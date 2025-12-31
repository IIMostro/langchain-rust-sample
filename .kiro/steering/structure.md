# Project Structure

```
langchain-rust-sample/
├── Cargo.toml          # 项目配置和依赖声明
├── Cargo.lock          # 依赖锁定文件
├── src/
│   └── main.rs         # 程序入口
└── target/             # 构建输出目录（git 忽略）
```

## 目录说明

| 目录/文件 | 说明 |
|---|---|
| `src/` | 源代码目录 |
| `src/main.rs` | 程序主入口 |
| `Cargo.toml` | Rust 项目配置 |
| `target/` | 编译产物，不纳入版本控制 |

## 代码组织约定

- 主程序逻辑放在 `src/main.rs`
- 随着项目扩展，可按模块拆分到 `src/` 下的子模块
