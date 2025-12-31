# TextSplitter 文本分割器说明

## 概述

`TextSplitter` 是 `langchain-rust` 提供的文本分割 trait，用于将长文本或文档分割成较小的块，便于嵌入和向量存储。`TokenSplitter` 是基于 token 的具体实现。

## 来源

- 库: `langchain-rust` (版本 4.6.0)
- 路径:
  - `langchain_rust::text_splitter::TextSplitter` (trait)
  - `langchain_rust::text_splitter::TokenSplitter` (实现)
  - `langchain_rust::text_splitter::SplitterOptions` (配置)

---

## TextSplitter Trait

### 定义

```rust
#[async_trait]
pub trait TextSplitter: Send + Sync {
    /// 分割单个文本字符串
    async fn split_text(&self, text: &str) -> Result<Vec<String>, TextSplitterError>;

    /// 分割多个文档，保留元数据
    async fn split_documents(
        &self,
        documents: &[Document],
    ) -> Result<Vec<Document>, TextSplitterError>;

    /// 从文本和元数据创建文档
    async fn create_documents(
        &self,
        text: &[String],
        metadatas: &[HashMap<String, Value>],
    ) -> Result<Vec<Document>, TextSplitterError>;
}
```

### 方法说明

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `split_text` | `&str` | `Vec<String>` | 将单个文本分割成多个块 |
| `split_documents` | `&[Document]` | `Vec<Document>` | 分割文档列表，保留每个文档的元数据 |
| `create_documents` | `&[String]`, `&[HashMap]` | `Vec<Document>` | 从文本和元数据创建分割后的文档 |

---

## SplitterOptions

分割器的配置选项。

### 定义

```rust
pub struct SplitterOptions {
    pub chunk_size: usize,      // 每个块的最大 token 数
    pub chunk_overlap: usize,   // 块之间的重叠 token 数
    pub model_name: String,     // 模型名称（用于 tokenizer）
    pub encoding_name: String,  // 编码名称
    pub trim_chunks: bool,      // 是否修剪块的空白
}
```

### 默认值

| 字段 | 默认值 | 说明 |
|------|--------|------|
| `chunk_size` | 512 | 每个块的最大 token 数 |
| `chunk_overlap` | 0 | 块之间无重叠 |
| `model_name` | `"gpt-3.5-turbo"` | 默认模型 |
| `encoding_name` | `"cl100k_base"` | 默认编码 |
| `trim_chunks` | false | 不修剪空白 |

### 构建方法

```rust
use langchain_rust::text_splitter::SplitterOptions;

let options = SplitterOptions::new()
    .with_chunk_size(512)       // 每块最大 512 个 token
    .with_chunk_overlap(50)     // 块之间重叠 50 个 token
    .with_trim_chunks(true)     // 修剪空白
    .with_encoding_name("cl100k_base");
```

### 支持的编码 (encoding_name)

| 编码名称 | 说明 |
|----------|------|
| `o200k_base` | GPT-4o 系列使用的编码 |
| `cl100k_base` | GPT-4, GPT-3.5-turbo 使用的编码（默认） |
| `p50k_base` | Codex 模型使用的编码 |
| `r50k_base` | GPT-3 模型使用的编码 |
| `p50k_edit` | 编辑模型使用的编码 |
| `gpt2` | GPT-2 使用的编码 |

### 支持的模型 (model_name)

当设置 `model_name` 时，会自动选择对应的 tokenizer。

#### Chat 模型（使用 o200k_base）

| 模型名称 | 上下文大小 |
|----------|-----------|
| `gpt-4o` | 128,000 |
| `gpt-4o-2024-05-13` | 128,000 |

#### Chat 模型（使用 cl100k_base）

| 模型名称 | 上下文大小 |
|----------|-----------|
| `gpt-4` | 8,192 |
| `gpt-4-32k` | 32,768 |
| `gpt-4-0125-preview` | 128,000 |
| `gpt-4-1106-preview` | 128,000 |
| `gpt-3.5-turbo` | 16,385 |
| `gpt-3.5-turbo-16k` | 16,385 |
| `gpt-3.5-turbo-0125` | 16,385 |
| `gpt-3.5-turbo-1106` | 16,385 |

#### 微调模型（使用 cl100k_base）

| 模型名称 |
|----------|
| `ft:gpt-4` |
| `ft:gpt-3.5-turbo` |
| `ft:davinci-002` |
| `ft:babbage-002` |

#### Text 模型（使用 p50k_base）

| 模型名称 | 上下文大小 |
|----------|-----------|
| `text-davinci-003` | 4,097 |
| `text-davinci-002` | 4,097 |

#### Text 模型（使用 r50k_base）

| 模型名称 | 上下文大小 |
|----------|-----------|
| `text-davinci-001` | 2,049 |
| `text-curie-001` | 2,049 |
| `text-babbage-001` | 2,049 |
| `text-ada-001` | 2,049 |
| `davinci` | 2,049 |
| `curie` | 2,049 |
| `babbage` | 2,049 |
| `ada` | 2,049 |

#### Code 模型（使用 p50k_base）

| 模型名称 | 上下文大小 |
|----------|-----------|
| `code-davinci-002` | 8,001 |
| `code-davinci-001` | - |
| `code-cushman-002` | - |
| `code-cushman-001` | 2,048 |
| `davinci-codex` | - |
| `cushman-codex` | - |

#### Edit 模型（使用 p50k_edit）

| 模型名称 |
|----------|
| `text-davinci-edit-001` |
| `code-davinci-edit-001` |

#### Embedding 模型（使用 cl100k_base）

| 模型名称 | 上下文大小 |
|----------|-----------|
| `text-embedding-ada-002` | 8,192 |
| `text-embedding-3-small` | - |
| `text-embedding-3-large` | - |

#### 旧版 Embedding 模型（使用 r50k_base）

| 模型名称 |
|----------|
| `text-similarity-davinci-001` |
| `text-similarity-curie-001` |
| `text-similarity-babbage-001` |
| `text-similarity-ada-001` |
| `text-search-davinci-doc-001` |
| `text-search-curie-doc-001` |
| `text-search-babbage-doc-001` |
| `text-search-ada-doc-001` |
| `code-search-babbage-code-001` |
| `code-search-ada-code-001` |

#### 开源模型

| 模型名称 | Tokenizer |
|----------|-----------|
| `gpt2` | Gpt2 |

### model_name 与 encoding_name 的关系

- 如果同时设置了 `model_name` 和 `encoding_name`，优先使用 `encoding_name`
- 如果只设置 `model_name`，会自动根据模型选择对应的 tokenizer
- 默认使用 `cl100k_base` 编码

---

## TokenSplitter

基于 token 的文本分割器实现。

### 构造方法

```rust
use langchain_rust::text_splitter::{SplitterOptions, TokenSplitter};

// 1. 使用默认配置
let splitter = TokenSplitter::default();

// 2. 使用自定义配置
let options = SplitterOptions::new()
    .with_chunk_size(256)
    .with_chunk_overlap(20);

let splitter = TokenSplitter::new(options);
```

---

## 使用示例

### 分割单个文本

```rust
use langchain_rust::text_splitter::{SplitterOptions, TextSplitter, TokenSplitter};

async fn split_text_example() -> anyhow::Result<()> {
    let options = SplitterOptions::new()
        .with_chunk_size(100)
        .with_chunk_overlap(10);

    let splitter = TokenSplitter::new(options);

    let text = "这是一段很长的文本...";
    let chunks = splitter.split_text(text).await?;

    for (i, chunk) in chunks.iter().enumerate() {
        println!("块 {}: {}", i + 1, chunk);
    }

    Ok(())
}
```

### 分割文档列表

```rust
use langchain_rust::{
    schemas::Document,
    text_splitter::{SplitterOptions, TextSplitter, TokenSplitter},
};

async fn split_documents_example() -> anyhow::Result<()> {
    let splitter = TokenSplitter::new(
        SplitterOptions::new()
            .with_chunk_size(512)
            .with_chunk_overlap(50)
    );

    let docs = vec![
        Document::new("第一个文档的内容..."),
        Document::new("第二个文档的内容..."),
    ];

    // 分割后每个块会保留原文档的元数据
    let split_docs = splitter.split_documents(&docs).await?;

    println!("原始文档数: {}", docs.len());
    println!("分割后文档数: {}", split_docs.len());

    Ok(())
}
```

---

## RAG 完整流程示例

将 PDF 文档分割、向量化并存入 Qdrant 的完整示例：

```rust
use anyhow::{anyhow, Result};
use langchain_rust::{
    document_loaders::{pdf_extract_loader::PdfExtractLoader, Loader},
    embedding::OllamaEmbedder,
    text_splitter::{SplitterOptions, TextSplitter, TokenSplitter},
    vectorstore::{qdrant::{Qdrant, StoreBuilder}, VecStoreOptions, VectorStore},
};
use tokio_stream::StreamExt;

async fn rag_pipeline() -> Result<()> {
    // 1. 加载 PDF 文档
    let path = "/path/to/document.pdf";
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
    let client = Qdrant::from_url("http://localhost:6334")
        .build()
        .map_err(|e| anyhow!("连接 Qdrant 失败: {}", e))?;

    // 6. 构建向量存储
    let store = StoreBuilder::new()
        .embedder(embedder)
        .client(client)
        .collection_name("my-documents")
        .recreate_collection(true)
        .build()
        .await
        .map_err(|e| anyhow!("构建向量存储失败: {}", e))?;

    // 7. 将分割后的文档添加到向量数据库
    let ids = store
        .add_documents(&split_docs, &VecStoreOptions::default())
        .await
        .map_err(|e| anyhow!("添加文档失败: {}", e))?;

    println!("成功添加 {} 个文档块", ids.len());

    // 8. 相似度搜索
    let query = "搜索关键词";
    let results = store
        .similarity_search(query, 3, &VecStoreOptions::default())
        .await
        .map_err(|e| anyhow!("搜索失败: {}", e))?;

    println!("\n搜索 '{}' 的结果:", query);
    for (i, doc) in results.iter().enumerate() {
        println!("--- 结果 {} (分数: {:.4}) ---", i + 1, doc.score);
        println!("{}", doc.page_content);
    }

    Ok(())
}
```

---

## 分割策略建议

### chunk_size 选择

| 场景 | 建议值 | 说明 |
|------|--------|------|
| 问答系统 | 256-512 | 较小的块便于精确匹配 |
| 文档摘要 | 1024-2048 | 较大的块保留更多上下文 |
| 代码分析 | 512-1024 | 保持函数/类的完整性 |

### chunk_overlap 选择

| 场景 | 建议值 | 说明 |
|------|--------|------|
| 无上下文依赖 | 0 | 独立的信息块 |
| 一般文档 | 50-100 | 保持句子连贯性 |
| 强上下文依赖 | 100-200 | 保持段落连贯性 |

---

## 错误处理

### TextSplitterError

```rust
pub enum TextSplitterError {
    MetadataTextMismatch,  // 元数据和文本数量不匹配
    TokenizerNotFound,     // 找不到指定的 tokenizer
    InvalidTokenizer,      // 无效的 tokenizer
    InvalidModel,          // 无效的模型名称
    // ...
}
```

---

## 其他分割器

### MarkdownSplitter

专门用于分割 Markdown 文档，按标题层级分割。

```rust
use langchain_rust::text_splitter::MarkdownSplitter;

let splitter = MarkdownSplitter::new(options);
```

---

## 依赖要求

```toml
[dependencies]
langchain-rust = { version = "4.6.0", features = ["ollama", "qdrant", "pdf-extract"] }
```

## 参考资料

- [langchain-rust text_splitter 模块](https://docs.rs/langchain-rust/latest/langchain_rust/text_splitter/index.html)
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) - Token 计数库
