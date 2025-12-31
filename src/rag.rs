#[cfg(test)]
mod tests {

    use anyhow::{anyhow, Result};
    use langchain_rust::{
        document_loaders::{pdf_extract_loader::PdfExtractLoader, Loader},
        embedding::OllamaEmbedder,
        text_splitter::{SplitterOptions, TextSplitter, TokenSplitter},
        vectorstore::{qdrant::{Qdrant, StoreBuilder}, VecStoreOptions, VectorStore},
    };
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn test_read_pdf_should_work() -> Result<()> {
        let path = "/mnt/d/Downloads/中华人民共和国劳动法.pdf";
        let loader = PdfExtractLoader::from_path(path).expect("Failed to create PdfExtractLoader");
        let docs = loader
            .load()
            .await
            .unwrap()
            .map(|d| d.unwrap())
            .collect::<Vec<_>>()
            .await;
        for doc in docs {
            println!("doc: {:?}", doc);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_pdf_content_embedding_should_work() -> Result<()> {
        // 1. 加载 PDF 文档
        let path = "/mnt/d/Downloads/中华人民共和国劳动法.pdf";
        let loader = PdfExtractLoader::from_path(path)
            .map_err(|e| anyhow!("创建 PdfExtractLoader 失败: {}", e))?;

        let docs = loader
            .load()
            .await
            .map_err(|e| anyhow!("加载 PDF 失败: {}", e))?
            .map(|d| d.unwrap())
            .collect::<Vec<_>>()
            .await;

        println!("加载了 {} 个文档", docs.len());

        // 2. 配置文本分割器
        //    - chunk_size: 每个分块的最大 token 数
        //    - chunk_overlap: 分块之间的重叠 token 数，用于保持上下文连贯性
        let splitter_options = SplitterOptions::new()
            .with_chunk_size(512)
            .with_chunk_overlap(50)
            .with_trim_chunks(true);

        let splitter = TokenSplitter::new(splitter_options);

        // 3. 分割文档
        let split_docs = splitter
            .split_documents(&docs)
            .await
            .map_err(|e| anyhow!("分割文档失败: {}", e))?;

        println!("分割后得到 {} 个文档块", split_docs.len());

        // 4. 创建嵌入器
        let embedder = OllamaEmbedder::default().with_model("bge-m3:567m");

        // 5. 连接 Qdrant 向量数据库
        let client = Qdrant::from_url("http://192.168.0.153:6334")
            .build()
            .map_err(|e| anyhow!("连接 Qdrant 失败: {}", e))?;

        // 6. 构建向量存储
        let store = StoreBuilder::new()
            .embedder(embedder)
            .client(client)
            .collection_name("labor-law")
            .recreate_collection(true) // 重建集合，清除旧数据
            .build()
            .await
            .map_err(|e| anyhow!("构建向量存储失败: {}", e))?;

        // 7. 将分割后的文档添加到向量数据库
        let ids = store
            .add_documents(&split_docs, &VecStoreOptions::default())
            .await
            .map_err(|e| anyhow!("添加文档到向量数据库失败: {}", e))?;

        println!("成功添加 {} 个文档块到向量数据库", ids.len());

        // 8. 测试相似度搜索
        let query = "劳动合同";
        let results = store
            .similarity_search(query, 3, &VecStoreOptions::default())
            .await
            .map_err(|e| anyhow!("相似度搜索失败: {}", e))?;

        println!("\n搜索 '{}' 的结果:", query);
        for (i, doc) in results.iter().enumerate() {
            println!("--- 结果 {} (分数: {:.4}) ---", i + 1, doc.score);
            println!("{}", doc.page_content);
        }

        Ok(())
    }
}
