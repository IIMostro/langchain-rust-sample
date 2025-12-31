# Embedder 与 OllamaEmbedder 说明

## 概述

`Embedder` 是 `langchain-rust` 定义的嵌入向量生成 trait，`OllamaEmbedder` 是基于 Ollama 的具体实现，用于将文本转换为向量表示，常用于语义搜索、相似度计算等场景。

## 来源

- 库: `langchain-rust` (版本 4.6.0)
- 路径: 
  - `langchain_rust::embedding::Embedder` (trait)
  - `langchain_rust::embedding::OllamaEmbedder` (实现)

---

## Embedder Trait

### 定义

```rust
#[async_trait]
pub trait Embedder: Send + Sync {
    /// 批量嵌入多个文档
    async fn embed_documents(&self, documents: &[String]) -> Result<Vec<Vec<f64>>, EmbedderError>;
    
    /// 嵌入单个查询文本
    async fn embed_query(&self, text: &str) -> Result<Vec<f64>, EmbedderError>;
}
```

### 方法说明

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `embed_documents` | `&[String]` | `Vec<Vec<f64>>` | 批量将多个文档转换为向量，返回向量列表 |
| `embed_query` | `&str` | `Vec<f64>` | 将单个查询文本转换为向量 |

### 设计说明

- `embed_documents`: 用于批量处理文档，适合建立向量索引
- `embed_query`: 用于处理用户查询，适合搜索时使用
- 两者分开是因为某些嵌入模型对文档和查询有不同的处理方式

---

## OllamaEmbedder

### 结构体定义

```rust
pub struct OllamaEmbedder {
    pub(crate) client: Arc<OllamaClient>,           // ollama-rs 客户端
    pub(crate) model: String,                        // 嵌入模型名称
    pub(crate) options: Option<GenerationOptions>,   // 生成选项
}
```

### 默认配置

- 默认模型: `nomic-embed-text` (137M 参数，约 274MB)
- 默认连接: `localhost:11434`
- 输出维度: 768 维（nomic-embed-text）

### 构造方法

#### `default()`

使用默认配置创建嵌入器。

```rust
use langchain_rust::embedding::OllamaEmbedder;

let embedder = OllamaEmbedder::default();
```

#### `new()`

使用自定义配置创建嵌入器。

```rust
use langchain_rust::embedding::OllamaEmbedder;
use langchain_rust::llm::client::GenerationOptions;
use ollama_rs::Ollama as OllamaClient;
use std::sync::Arc;

let client = Arc::new(OllamaClient::new("http://localhost", 11434));
let options = GenerationOptions::default().temperature(0.5);
let embedder = OllamaEmbedder::new(client, "nomic-embed-text", Some(options));
```

### Builder 方法

#### `with_model()`

设置嵌入模型。

```rust
let embedder = OllamaEmbedder::default()
    .with_model("mxbai-embed-large");
```

#### `with_options()`

设置生成选项。

```rust
use langchain_rust::llm::client::GenerationOptions;

let options = GenerationOptions::default().temperature(0.5);
let embedder = OllamaEmbedder::default()
    .with_options(options);
```

---

## 使用示例

### 嵌入单个查询

```rust
use langchain_rust::embedding::{Embedder, OllamaEmbedder};

async fn embed_single() -> anyhow::Result<()> {
    let embedder = OllamaEmbedder::default();
    
    let query = "什么是 Rust 的所有权系统？";
    let embedding = embedder.embed_query(query).await?;
    
    println!("向量维度: {}", embedding.len());  // 768
    println!("前 5 个值: {:?}", &embedding[..5]);
    
    Ok(())
}
```

### 批量嵌入文档

```rust
use langchain_rust::embedding::{Embedder, OllamaEmbedder};

async fn embed_batch() -> anyhow::Result<()> {
    let embedder = OllamaEmbedder::default();
    
    let documents = vec![
        "Rust 是一门系统编程语言".to_string(),
        "Rust 的所有权系统保证内存安全".to_string(),
        "Rust 没有垃圾回收器".to_string(),
    ];
    
    let embeddings = embedder.embed_documents(&documents).await?;
    
    println!("文档数量: {}", embeddings.len());  // 3
    for (i, emb) in embeddings.iter().enumerate() {
        println!("文档 {} 向量维度: {}", i, emb.len());
    }
    
    Ok(())
}
```

### 计算相似度

```rust
use langchain_rust::embedding::{Embedder, OllamaEmbedder};

/// 计算余弦相似度
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    dot / (norm_a * norm_b)
}

async fn similarity_search() -> anyhow::Result<()> {
    let embedder = OllamaEmbedder::default();
    
    // 1. 嵌入文档
    let documents = vec![
        "Rust 是一门系统编程语言".to_string(),
        "Python 是一门脚本语言".to_string(),
        "Rust 的内存安全特性".to_string(),
    ];
    let doc_embeddings = embedder.embed_documents(&documents).await?;
    
    // 2. 嵌入查询
    let query = "Rust 语言的特点";
    let query_embedding = embedder.embed_query(query).await?;
    
    // 3. 计算相似度
    for (i, doc_emb) in doc_embeddings.iter().enumerate() {
        let similarity = cosine_similarity(&query_embedding, doc_emb);
        println!("文档 {}: 相似度 = {:.4}", i, similarity);
    }
    
    Ok(())
}
```

---

## 常用嵌入模型

| 模型 | 参数量 | 维度 | 说明 |
|------|--------|------|------|
| `nomic-embed-text` | 137M | 768 | 默认模型，通用文本嵌入 |
| `mxbai-embed-large` | 335M | 1024 | 高质量嵌入，适合复杂语义 |
| `all-minilm` | 23M | 384 | 轻量级模型，速度快 |
| `snowflake-arctic-embed` | 110M | 768 | 多语言支持 |

下载模型：
```bash
ollama pull nomic-embed-text
ollama pull mxbai-embed-large
```

---

## 错误处理

### EmbedderError

```rust
pub enum EmbedderError {
    RequestError(ReqwestError),      // 网络请求失败
    OpenAIError(OpenAIError),        // OpenAI 错误
    UrlParseError(url::ParseError),  // URL 解析错误
    HttpError { status_code, error_message },  // HTTP 错误
    FastEmbedError(String),          // FastEmbed 错误
    OllamaError(OllamaError),        // Ollama 错误
    // ...
}
```

### 错误处理示例

```rust
use langchain_rust::embedding::{Embedder, EmbedderError, OllamaEmbedder};

async fn handle_error() {
    let embedder = OllamaEmbedder::default();
    
    match embedder.embed_query("测试文本").await {
        Ok(embedding) => println!("成功: {} 维", embedding.len()),
        Err(EmbedderError::OllamaError(e)) => {
            eprintln!("Ollama 错误: {}", e);
            // 可能是连接失败或模型不存在
        }
        Err(e) => eprintln!("其他错误: {}", e),
    }
}
```

---

## 与向量数据库集成

`OllamaEmbedder` 通常与向量数据库配合使用：

```rust
use langchain_rust::embedding::{Embedder, OllamaEmbedder};
// 假设使用 Qdrant 向量数据库

async fn vector_store_example() -> anyhow::Result<()> {
    let embedder = OllamaEmbedder::default();
    
    // 1. 准备文档
    let documents = vec![
        "文档内容 1".to_string(),
        "文档内容 2".to_string(),
    ];
    
    // 2. 生成嵌入向量
    let embeddings = embedder.embed_documents(&documents).await?;
    
    // 3. 存入向量数据库（伪代码）
    // for (doc, emb) in documents.iter().zip(embeddings.iter()) {
    //     vector_store.insert(doc, emb).await?;
    // }
    
    // 4. 查询时
    let query_embedding = embedder.embed_query("查询内容").await?;
    // let results = vector_store.search(&query_embedding, top_k=5).await?;
    
    Ok(())
}
```

---

## 依赖要求

1. 本地运行 Ollama 服务
2. 已下载嵌入模型（如 `ollama pull nomic-embed-text`）
3. Cargo.toml 配置：

```toml
[dependencies]
langchain-rust = { version = "4.6.0", features = ["ollama"] }
```

## 参考资料

- [Ollama Embedding 模型](https://ollama.com/search?c=embedding)
- [nomic-embed-text](https://ollama.com/library/nomic-embed-text)
- [langchain-rust 源码](https://github.com/Abraxas-365/langchain-rust)
