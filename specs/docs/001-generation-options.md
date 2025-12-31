# GenerationOptions 参数说明

## 概述

`GenerationOptions` 是 `ollama-rs` 库提供的生成选项配置结构体，用于控制 Ollama 模型的文本生成行为。在 `langchain-rust` 中通过 `langchain_rust::llm::client::GenerationOptions` 导出使用。

## 来源

- 库: `ollama-rs` (版本 0.2.2)
- 路径: `ollama_rs::generation::options::GenerationOptions`
- 在 langchain-rust 中的导出: `langchain_rust::llm::client::GenerationOptions`

## 使用方式

```rust
use langchain_rust::llm::client::{GenerationOptions, Ollama};

let options = GenerationOptions::default()
    .temperature(0.7)
    .num_gpu(2)
    .num_predict(256);

let ollama = Ollama::default()
    .with_model("deepseek-r1:8b")
    .with_options(options);
```

## 参数详解

### 采样控制参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `temperature` | `f32` | 0.8 | 模型温度。值越高，生成结果越有创意；值越低，结果越确定性 |
| `top_k` | `u32` | 40 | 从概率最高的 K 个 token 中采样。值越高结果越多样，值越低越保守 |
| `top_p` | `f32` | 0.9 | 核采样参数，与 top_k 配合使用。值越高文本越多样，值越低越集中保守 |
| `seed` | `i32` | 0 | 随机数种子。设置固定值可使相同 prompt 生成相同结果 |

### Mirostat 采样参数

Mirostat 是一种用于控制困惑度的采样算法。

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `mirostat` | `u8` | 0 | 启用 Mirostat 采样。0=禁用，1=Mirostat，2=Mirostat 2.0 |
| `mirostat_eta` | `f32` | 0.1 | 学习率，影响算法对生成文本反馈的响应速度。值越低调整越慢，值越高响应越快 |
| `mirostat_tau` | `f32` | 5.0 | 控制输出的连贯性和多样性平衡。值越低文本越集中连贯 |

### 重复控制参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `repeat_last_n` | `i32` | 64 | 模型回看多少个 token 来防止重复。0=禁用，-1=使用 num_ctx |
| `repeat_penalty` | `f32` | 1.1 | 重复惩罚强度。值越高（如 1.5）惩罚越强，值越低（如 0.9）越宽松 |

### 上下文与生成长度参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `num_ctx` | `u32` | 2048 | 上下文窗口大小，用于生成下一个 token |
| `num_predict` | `i32` | 128 | 生成文本时预测的最大 token 数。-1=无限生成，-2=填满上下文 |
| `stop` | `Vec<String>` | - | 停止序列列表。遇到这些模式时 LLM 停止生成并返回 |

### 硬件配置参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `num_gpu` | `u32` | - | 发送到 GPU 的层数。macOS 默认为 1 以启用 Metal 支持，0 为禁用 |
| `num_thread` | `u32` | - | 计算时使用的线程数。默认由 Ollama 自动检测。建议设置为物理 CPU 核心数 |
| `num_gqa` | `u32` | - | Transformer 层中的 GQA 组数。某些模型需要设置，如 llama2:70b 需要设置为 8 |

### 其他参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `tfs_z` | `f32` | 1.0 | 尾部自由采样，用于减少低概率 token 的影响。值越高（如 2.0）影响越大，1.0 为禁用 |

## 常用配置示例

### 创意写作配置

```rust
// 高温度、高多样性，适合创意写作
let options = GenerationOptions::default()
    .temperature(1.0)
    .top_k(100)
    .top_p(0.95);
```

### 代码生成配置

```rust
// 低温度、低多样性，适合代码生成
let options = GenerationOptions::default()
    .temperature(0.2)
    .top_k(10)
    .top_p(0.5)
    .repeat_penalty(1.2);
```

### 长文本生成配置

```rust
// 扩大上下文窗口和生成长度
let options = GenerationOptions::default()
    .num_ctx(4096)
    .num_predict(1024);
```

### 可复现结果配置

```rust
// 固定种子以获得可复现的结果
let options = GenerationOptions::default()
    .seed(42)
    .temperature(0.7);
```

## 参考资料

- [Ollama API 文档](https://github.com/ollama/ollama/blob/main/docs/api.md)
- [ollama-rs 源码](https://github.com/pepperoni21/ollama-rs)
- [langchain-rust 源码](https://github.com/Abraxas-365/langchain-rust)
