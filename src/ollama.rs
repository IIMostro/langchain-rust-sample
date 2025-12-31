#[cfg(test)]
mod tests {
    use anyhow::Result;
    use langchain_rust::{
        language_models::llm::LLM,
        llm::client::{GenerationOptions, Ollama},
        schemas::Message,
    };
    use tokio::io::AsyncWriteExt;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn test_ollama_should_work() -> Result<()> {
        let options = GenerationOptions::default().num_ctx(4096).num_predict(1024);
        let ollama = Ollama::default()
            .with_model("deepseek-r1:8b")
            .with_options(options);
        let response = ollama.invoke("Hi").await.unwrap();
        println!("{}", response);
        Ok(())
    }

    #[tokio::test]
    async fn test_async_generate_message_shold_work() -> Result<()> {
        let ollama = Ollama::default().with_model("deepseek-r1:8b");

        let messages = vec![Message::new_human_message("你好，请介绍一下 Rust 语言")];

        let result = ollama.generate(&messages).await?;
        println!("生成内容: {}", result.generation);

        // 可选：获取 token 使用量
        if let Some(tokens) = result.tokens {
            println!("Prompt tokens: {}", tokens.prompt_tokens);
            println!("Completion tokens: {}", tokens.completion_tokens);
            println!("Total tokens: {}", tokens.total_tokens);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_stream_message_should_work() -> Result<()> {
        let ollama = Ollama::default().with_model("deepseek-r1:8b");

        let messages = vec![Message::new_human_message("为什么水在 100 度沸腾？")];

        let mut stream = ollama.stream(&messages).await?;
        let mut stdout = tokio::io::stdout();

        while let Some(result) = stream.next().await {
            let data = result?;
            stdout.write(data.content.as_bytes()).await?;
        }
        stdout.write(b"\n").await?;
        stdout.flush().await?;
        Ok(())
    }
}
