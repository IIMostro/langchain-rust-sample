# langchain-rust-sample

基于 LangChain Rust 的示例项目，用于探索和演示 LangChain 在 Rust 生态中的使用方式。

## 功能

- Ollama 本地 LLM 推理
- Qdrant 向量数据库集成
- 文本嵌入与语义搜索

## 依赖

- [Rust](https://www.rust-lang.org/) (Edition 2024)
- [Ollama](https://ollama.com/) - 本地 LLM 服务
- [Qdrant](https://qdrant.tech/) - 向量数据库

## 快速开始

### 1. 启动 Ollama

```bash
# 安装后拉取模型
ollama pull llama3.2
ollama pull bge-m3:567m
```

### 2. 启动 Qdrant

```bash
docker run -p 6333:6333 -p 6334:6334 qdrant/qdrant
```

### 3. 运行项目

```bash
cargo build
cargo run
```

## 项目结构

```
langchain-rust-sample/
├── src/
│   ├── main.rs          # 程序入口
│   ├── ollama.rs        # Ollama LLM 示例
│   └── embedding.rs     # 嵌入与向量搜索示例
├── specs/docs/          # API 文档
│   ├── 001-generation-options.md
│   ├── 002-ollama-client.md
│   ├── 003-embedder.md
│   └── 004-qdrant-vectorstore.md
├── Cargo.toml
└── README.md
```

## 文档

详细的 API 文档请参考 `specs/docs/` 目录。

## 许可证

MIT
