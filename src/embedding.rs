#[cfg(test)]
mod tests {

    

    use anyhow::Result;
    use anyhow::anyhow;
    use langchain_rust::schemas::Document;
    use langchain_rust::vectorstore::VecStoreOptions;
    use langchain_rust::vectorstore::VectorStore;
    use langchain_rust::{
        embedding::{Embedder, OllamaEmbedder},
        vectorstore::qdrant::{Qdrant, StoreBuilder},
    };

    #[tokio::test]
    async fn test_embedding_should_work() -> Result<()> {
        let ollama = OllamaEmbedder::default().with_model("bge-m3:567m");
        let response = ollama.embed_query("Why is the sky blue?").await.unwrap();
        println!("{:?}", response);
        Ok(())
    }

    #[tokio::test]
    async fn test_qdrant_should_work() -> Result<()> {
        let embedder = OllamaEmbedder::default().with_model("bge-m3:567m");
        let client = Qdrant::from_url("http://192.168.0.153:6334").build().unwrap();
        let store = StoreBuilder::new()
            .embedder(embedder)
            .client(client)
            .collection_name("langchain-rust-sample")
            .build()
            .await
            .map_err(|e| anyhow!("build qdrant storage error!{}", e))?;
        // Add documents to the database
        let doc1 = Document::new("李白");
        let doc2 = Document::new("杜甫");
        let doc3 = Document::new("唐朝");
        let doc4 = Document::new("苏轼");

        store
            .add_documents(&vec![doc1, doc2, doc3, doc4], &VecStoreOptions::default())
            .await
            .unwrap();
        let mut query = String::from("李清照");
        std::io::stdin().read_line(&mut query).unwrap();
        let results = store
            .similarity_search(&query, 2, &VecStoreOptions::default())
            .await
            .unwrap();
        if results.is_empty() {
            println!("No results found.");
        } else {
            results.iter().for_each(|r| {
                println!("Document: {}, score:{}", r.page_content, r.score);
            });
        }
        Ok(())
    }
}
