# Tech Stack

## 语言与版本

- **Rust**: Edition 2024
- **包管理**: Cargo

## 核心依赖

| 库 | 版本 | 用途 |
|---|---|---|
| langchain-rust | 4.6.0 | LangChain 核心框架 |
| anyhow | 1.0.100 | 错误处理 |
| serde | 1.0.228 | 序列化/反序列化 |
| serde_json | 1.0.148 | JSON 处理 |

## LangChain Features

- `ollama`: 本地 LLM 推理支持
- `qdrant`: 向量数据库集成

## 常用命令

```bash
# 构建项目
cargo build

# 运行项目
cargo run

# 运行测试
cargo test

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 代码检查
cargo clippy
```

## 外部服务依赖

- **Ollama**: 本地 LLM 服务
- **Qdrant**: 向量数据库服务
