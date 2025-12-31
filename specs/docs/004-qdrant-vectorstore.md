# Qdrant VectorStore 说明

## 概述

本文档介绍 `langchain-rust` 中与 Qdrant 向量数据库集成相关的类型，包括 `Document`、`VecStoreOptions`、`VectorStore` trait、`StoreBuilder` 和 `Store`。

## 来源

- 库: `langchain-rust` (版本 4.6.0)
- 路径:
  - `langchain_rust::schemas::Document`
  - `langchain_rust::vectorstore::VecStoreOptions`
  - `langchain_rust::vectorstore::VectorStore` (trait)
  - `langchain_rust::vectorstore::qdrant::{Qdrant, StoreBuilder, Store}`

---

## Document

文档结构体，表示一个带有内容、元数据和相关性分数的文档。

### 定义

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub page_content: String,              // 文档内容
    pub metadata: HashMap<String, Value>,  // 元数据（键值对）
    pub score: f64,                         // 相关性分数
}
```

### 构造方法

```rust
use langchain_rust::schemas::Document;
use serde_json::json;
use std::collections::HashMap;

// 1. 简单创建
let doc = Document::new("这是文档内容");

// 2. 带元数据
let mut metadata = HashMap::new();
metadata.insert("author".to_string(), json!("张三"));
metadata.insert("category".to_string(), json!("技术"));

let doc = Document::new("Rust 是一门系统编程语言")
    .with_metadata(metadata)
    .with_score(0.95);

// 3. 默认空文档
let doc = Document::default();
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `page_content` | `String` | 文档的文本内容 |
| `metadata` | `HashMap<String, Value>` | 文档的元数据，可存储任意 JSON 值 |
| `score` | `f64` | 相似度搜索时的相关性分数（0.0 ~ 1.0） |

---

## VecStoreOptions

向量存储操作的选项配置。

### 定义

```rust
pub struct VecStoreOptions {
    pub name_space: Option<String>,        // 命名空间（Qdrant 不支持）
    pub score_threshold: Option<f32>,      // 分数阈值
    pub filters: Option<Value>,            // 过滤条件
    pub embedder: Option<Arc<dyn Embedder>>, // 嵌入器（可覆盖默认）
}
```

### 构造方法

```rust
use langchain_rust::vectorstore::VecStoreOptions;
use serde_json::json;

// 1. 默认选项
let options = VecStoreOptions::default();

// 2. 带分数阈值（只返回分数高于阈值的结果）
let options = VecStoreOptions::new()
    .with_score_threshold(0.7);

// 3. 使用自定义嵌入器
let options = VecStoreOptions::new()
    .with_embedder(my_embedder);
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `name_space` | `Option<String>` | 命名空间（注意：Qdrant 不支持此选项） |
| `score_threshold` | `Option<f32>` | 相似度分数阈值，低于此值的结果将被过滤 |
| `filters` | `Option<Value>` | JSON 格式的过滤条件（Qdrant 需使用 `search_filter`） |
| `embedder` | `Option<Arc<dyn Embedder>>` | 可选的嵌入器，用于覆盖 Store 默认的嵌入器 |

---

## VectorStore Trait

向量存储的核心 trait，定义了文档存储和相似度搜索的接口。

### 定义

```rust
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// 添加文档到向量存储
    async fn add_documents(
        &self,
        docs: &[Document],
        opt: &VecStoreOptions,
    ) -> Result<Vec<String>, Box<dyn Error>>;

    /// 相似度搜索
    async fn similarity_search(
        &self,
        query: &str,
        limit: usize,
        opt: &VecStoreOptions,
    ) -> Result<Vec<Document>, Box<dyn Error>>;
}
```

### 方法说明

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `add_documents` | `docs`, `opt` | `Vec<String>` | 添加文档，返回文档 ID 列表 |
| `similarity_search` | `query`, `limit`, `opt` | `Vec<Document>` | 相似度搜索，返回最相似的文档列表 |

### 便捷宏

```rust
use langchain_rust::{add_documents, similarity_search};

// 使用默认选项添加文档
add_documents!(store, &docs).await?;

// 使用默认选项搜索
let results = similarity_search!(store, "查询内容", 5).await?;
```

---

## StoreBuilder

Qdrant Store 的构建器，用于配置和创建 Store 实例。

### 定义

```rust
pub struct StoreBuilder {
    client: Option<Qdrant>,                // Qdrant 客户端
    embedder: Option<Arc<dyn Embedder>>,   // 嵌入器
    collection_name: Option<String>,       // 集合名称
    content_field: String,                 // 内容字段名（默认 "page_content"）
    metadata_field: String,                // 元数据字段名（默认 "metadata"）
    recreate_collection: bool,             // 是否重建集合
    search_filter: Option<Filter>,         // 搜索过滤器
}
```

### 构建方法

```rust
use langchain_rust::{
    embedding::OllamaEmbedder,
    vectorstore::qdrant::{Qdrant, StoreBuilder},
};

let embedder = OllamaEmbedder::default().with_model("bge-m3:567m");
let client = Qdrant::from_url("http://localhost:6334").build()?;

let store = StoreBuilder::new()
    .client(client)                              // 必需：Qdrant 客户端
    .embedder(embedder)                          // 必需：嵌入器
    .collection_name("my-collection")            // 必需：集合名称
    .content_field("content")                    // 可选：内容字段名
    .metadata_field("meta")                      // 可选：元数据字段名
    .recreate_collection(false)                  // 可选：是否重建集合
    .build()
    .await?;
```

### 方法说明

| 方法 | 参数 | 说明 |
|------|------|------|
| `client` | `Qdrant` | **必需**。Qdrant 客户端实例 |
| `embedder` | `impl Embedder` | **必需**。嵌入向量生成器 |
| `collection_name` | `&str` | **必需**。Qdrant 集合名称 |
| `content_field` | `&str` | 可选。存储文档内容的字段名，默认 `"page_content"` |
| `metadata_field` | `&str` | 可选。存储元数据的字段名，默认 `"metadata"` |
| `recreate_collection` | `bool` | 可选。是否删除并重建集合，默认 `false` |
| `search_filter` | `Filter` | 可选。搜索时的过滤条件 |
| `build` | - | 构建 Store 实例（异步） |

### 集合自动创建

如果指定的集合不存在，`build()` 会自动创建集合：
- 向量维度：根据嵌入器自动检测
- 距离度量：余弦相似度（Cosine）

---

## Store

Qdrant 向量存储的实现，实现了 `VectorStore` trait。

### 定义

```rust
pub struct Store {
    pub client: Qdrant,                    // Qdrant 客户端
    pub embedder: Arc<dyn Embedder>,       // 嵌入器
    pub collection_name: String,           // 集合名称
    pub content_field: String,             // 内容字段名
    pub metadata_field: String,            // 元数据字段名
    pub search_filter: Option<Filter>,     // 搜索过滤器
}
```

### 使用示例

```rust
use langchain_rust::{
    embedding::OllamaEmbedder,
    schemas::Document,
    vectorstore::{VecStoreOptions, VectorStore, qdrant::{Qdrant, StoreBuilder}},
};

async fn qdrant_example() -> anyhow::Result<()> {
    // 1. 创建嵌入器
    let embedder = OllamaEmbedder::default().with_model("bge-m3:567m");
    
    // 2. 创建 Qdrant 客户端
    let client = Qdrant::from_url("http://localhost:6334").build()?;
    
    // 3. 构建 Store
    let store = StoreBuilder::new()
        .embedder(embedder)
        .client(client)
        .collection_name("my-documents")
        .build()
        .await?;
    
    // 4. 添加文档
    let docs = vec![
        Document::new("Rust 是一门系统编程语言"),
        Document::new("Python 是一门脚本语言"),
        Document::new("Go 是 Google 开发的语言"),
    ];
    
    let ids = store
        .add_documents(&docs, &VecStoreOptions::default())
        .await?;
    println!("添加了 {} 个文档", ids.len());
    
    // 5. 相似度搜索
    let query = "系统级编程";
    let results = store
        .similarity_search(query, 2, &VecStoreOptions::default())
        .await?;
    
    for doc in results {
        println!("内容: {}, 分数: {:.4}", doc.page_content, doc.score);
    }
    
    Ok(())
}
```

---

## Qdrant 客户端

`Qdrant` 是 `qdrant_client` 库提供的客户端，用于连接 Qdrant 服务。

### 创建客户端

```rust
use langchain_rust::vectorstore::qdrant::Qdrant;

// 从 URL 创建
let client = Qdrant::from_url("http://localhost:6334")
    .build()?;

// 带 API Key（云服务）
let client = Qdrant::from_url("https://your-cluster.qdrant.io")
    .api_key("your-api-key")
    .build()?;
```

---

## Retriever

基于 VectorStore 的检索器，可用于 RAG 场景。

### 定义

```rust
pub struct Retriever {
    vstore: Box<dyn VectorStore>,
    num_docs: usize,
    options: VecStoreOptions,
}
```

### 使用示例

```rust
use langchain_rust::vectorstore::{Retriever, VecStoreOptions};

let retriever = Retriever::new(store, 5)  // 每次检索 5 个文档
    .with_options(VecStoreOptions::new().with_score_threshold(0.7));

// 实现了 schemas::Retriever trait
let docs = retriever.get_relevant_documents("查询内容").await?;
```

---

## 完整示例

```rust
use anyhow::{anyhow, Result};
use langchain_rust::{
    embedding::OllamaEmbedder,
    schemas::Document,
    vectorstore::{VecStoreOptions, VectorStore, qdrant::{Qdrant, StoreBuilder}},
};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 初始化嵌入器
    let embedder = OllamaEmbedder::default().with_model("bge-m3:567m");
    
    // 2. 连接 Qdrant
    let client = Qdrant::from_url("http://localhost:6334")
        .build()
        .map_err(|e| anyhow!("连接 Qdrant 失败: {}", e))?;
    
    // 3. 构建向量存储
    let store = StoreBuilder::new()
        .embedder(embedder)
        .client(client)
        .collection_name("poetry")
        .recreate_collection(true)  // 重建集合
        .build()
        .await
        .map_err(|e| anyhow!("构建 Store 失败: {}", e))?;
    
    // 4. 准备文档（带元数据）
    let docs = vec![
        Document::new("床前明月光，疑是地上霜").with_metadata({
            let mut m = HashMap::new();
            m.insert("author".to_string(), json!("李白"));
            m.insert("dynasty".to_string(), json!("唐"));
            m
        }),
        Document::new("春眠不觉晓，处处闻啼鸟").with_metadata({
            let mut m = HashMap::new();
            m.insert("author".to_string(), json!("孟浩然"));
            m.insert("dynasty".to_string(), json!("唐"));
            m
        }),
        Document::new("大江东去，浪淘尽，千古风流人物").with_metadata({
            let mut m = HashMap::new();
            m.insert("author".to_string(), json!("苏轼"));
            m.insert("dynasty".to_string(), json!("宋"));
            m
        }),
    ];
    
    // 5. 添加文档
    let ids = store
        .add_documents(&docs, &VecStoreOptions::default())
        .await?;
    println!("成功添加 {} 个文档", ids.len());
    
    // 6. 相似度搜索
    let query = "月亮";
    let options = VecStoreOptions::new().with_score_threshold(0.5);
    let results = store.similarity_search(query, 3, &options).await?;
    
    println!("\n搜索 '{}' 的结果:", query);
    for doc in results {
        println!("- {} (分数: {:.4})", doc.page_content, doc.score);
        if let Some(author) = doc.metadata.get("author") {
            println!("  作者: {}", author);
        }
    }
    
    Ok(())
}
```

---

## 注意事项

1. **Qdrant 不支持 namespace**: 使用 `VecStoreOptions::with_name_space()` 会报错
2. **过滤器**: Qdrant 不支持 JSON 格式的 `filters`，需使用 `StoreBuilder::search_filter()` 配置 `qdrant_client::qdrant::Filter`
3. **集合管理**: 建议提前在 Qdrant 中创建集合并配置好参数，而不是依赖自动创建
4. **向量维度**: 自动创建集合时会通过嵌入一段测试文本来检测维度

## 依赖要求

```toml
[dependencies]
langchain-rust = { version = "4.6.0", features = ["ollama", "qdrant"] }
```

## 参考资料

- [Qdrant 官方文档](https://qdrant.tech/documentation/)
- [qdrant-client Rust 库](https://docs.rs/qdrant-client)
- [langchain-rust 源码](https://github.com/Abraxas-365/langchain-rust)
