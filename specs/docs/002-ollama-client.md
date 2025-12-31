# Ollama Client 说明

## 概述

`Ollama` 是 `langchain-rust` 提供的 Ollama LLM 客户端，用于与本地 Ollama 服务进行交互，支持文本生成和流式输出。

## 来源

- 库: `langchain-rust` (版本 4.6.0)
- 路径: `langchain_rust::llm::client::Ollama`
- 底层依赖: `ollama-rs` (版本 0.2.2)

## 结构体定义

```rust
pub struct Ollama {
    pub(crate) client: Arc<OllamaClient>,  // ollama-rs 客户端实例
    pub(crate) model: String,               // 模型名称
    pub(crate) options: Option<GenerationOptions>,  // 生成选项
}
```

## 默认配置

- 默认模型: `llama3.2` (3B 参数，约 2.0GB)
- 默认连接: `localhost:11434`

## 构造方法

### `default()`

使用默认配置创建 Ollama 客户端。

```rust
use langchain_rust::llm::client::Ollama;

let ollama = Ollama::default();
// 等价于连接 localhost:11434，使用 llama3.2 模型
```

### `new()`

使用自定义配置创建 Ollama 客户端。

```rust
use langchain_rust::llm::client::{Ollama, GenerationOptions};
use ollama_rs::Ollama as OllamaClient;
use std::sync::Arc;

let client = Arc::new(OllamaClient::new("http://localhost", 11434));
let options = GenerationOptions::default().temperature(0.7);
let ollama = Ollama::new(client, "deepseek-r1:8b", Some(options));
```

## Builder 方法

### `with_model()`

设置使用的模型名称。

```rust
let ollama = Ollama::default()
    .with_model("deepseek-r1:8b");
```

### `with_options()`

设置生成选项。

```rust
use langchain_rust::llm::client::{Ollama, GenerationOptions};

let options = GenerationOptions::default()
    .temperature(0.7)
    .num_predict(256);

let ollama = Ollama::default()
    .with_model("llama3.2")
    .with_options(options);
```

## LLM Trait 实现

`Ollama` 实现了 `LLM` trait，提供以下核心方法：

### `generate()`

异步生成文本响应。

```rust
use langchain_rust::{
    language_models::llm::LLM,
    llm::client::Ollama,
    schemas::Message,
};

async fn example() -> anyhow::Result<()> {
    let ollama = Ollama::default().with_model("llama3.2");
    
    let messages = vec![
        Message::new_human_message("你好，请介绍一下 Rust 语言"),
    ];
    
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
```

### `invoke()`

简化的文本生成方法（LLM trait 提供的便捷方法）。

```rust
use langchain_rust::language_models::llm::LLM;

async fn example() -> anyhow::Result<()> {
    let ollama = Ollama::default().with_model("llama3.2");
    let response = ollama.invoke("什么是 Rust 的所有权系统？").await?;
    println!("{}", response);
    Ok(())
}
```

### `stream()`

流式生成文本，适合实时显示生成内容。

```rust
use langchain_rust::{
    language_models::llm::LLM,
    llm::client::Ollama,
    schemas::Message,
};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

async fn example() -> anyhow::Result<()> {
    let ollama = Ollama::default().with_model("llama3.2");
    
    let messages = vec![
        Message::new_human_message("为什么水在 100 度沸腾？"),
    ];
    
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
```

## 消息类型

支持的消息类型（`MessageType`）：

| 类型 | 说明 | 映射到 Ollama |
|------|------|---------------|
| `HumanMessage` | 用户消息 | `User` |
| `AIMessage` | AI 助手消息 | `Assistant` |
| `SystemMessage` | 系统提示消息 | `System` |
| `ToolMessage` | 工具调用消息 | `Assistant` |

## 图片支持

支持发送带图片的消息（多模态模型）：

```rust
use langchain_rust::schemas::{Message, ImageContent};

let message = Message::new_human_message("描述这张图片")
    .with_images(vec![
        ImageContent {
            image_url: "base64编码的图片数据".to_string(),
        }
    ]);
```

## 返回值类型

### `GenerateResult`

```rust
pub struct GenerateResult {
    pub generation: String,           // 生成的文本内容
    pub tokens: Option<TokenUsage>,   // token 使用统计（可选）
}
```

### `TokenUsage`

```rust
pub struct TokenUsage {
    pub prompt_tokens: u32,      // 输入 token 数
    pub completion_tokens: u32,  // 输出 token 数
    pub total_tokens: u32,       // 总 token 数
}
```

### `StreamData`

```rust
pub struct StreamData {
    pub content: String,  // 流式输出的文本片段
    // ... 其他字段
}
```

## 完整示例

```rust
use anyhow::Result;
use langchain_rust::{
    language_models::llm::LLM,
    llm::client::{GenerationOptions, Ollama},
    schemas::Message,
};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 配置生成选项
    let options = GenerationOptions::default()
        .temperature(0.7)
        .num_predict(512)
        .top_p(0.9);
    
    // 2. 创建 Ollama 客户端
    let ollama = Ollama::default()
        .with_model("deepseek-r1:8b")
        .with_options(options);
    
    // 3. 构建消息
    let messages = vec![
        Message::new_system_message("你是一个专业的 Rust 开发助手"),
        Message::new_human_message("请解释 Rust 的生命周期"),
    ];
    
    // 4. 生成响应
    let result = ollama.generate(&messages).await?;
    println!("{}", result.generation);
    
    Ok(())
}
```

## 错误处理

`Ollama` 的方法返回 `Result<T, LLMError>`，常见错误类型：

- `OllamaError`: 来自 ollama-rs 的错误（连接失败、模型不存在等）
- `ContentNotFound`: 响应中没有消息内容

```rust
use langchain_rust::language_models::LLMError;

match ollama.generate(&messages).await {
    Ok(result) => println!("{}", result.generation),
    Err(LLMError::OllamaError(e)) => eprintln!("Ollama 错误: {}", e),
    Err(e) => eprintln!("其他错误: {}", e),
}
```

## 依赖要求

使用 Ollama 客户端需要：

1. 本地运行 Ollama 服务（默认端口 11434）
2. 已下载所需模型（如 `ollama pull llama3.2`）
3. Cargo.toml 中启用 `ollama` feature：

```toml
[dependencies]
langchain-rust = { version = "4.6.0", features = ["ollama"] }
```

## 参考资料

- [langchain-rust 源码](https://github.com/Abraxas-365/langchain-rust)
- [Ollama 官网](https://ollama.com/)
- [Ollama 模型库](https://ollama.com/library)
