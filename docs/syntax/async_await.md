# async / await

Rust 用 **`async fn`** 定义异步函数，在函数体内用 **`.await`** 等待另一个异步操作完成。`async` 函数返回的是 **Future**（待执行的异步任务），本身不会立刻运行，需要 **异步运行时**（如 [Tokio](https://tokio.rs/)）来调度执行。

与 [多线程](/syntax/threads) 不同：async 适合大量 I/O 等待；线程适合 CPU 密集或需真并行。async 示例需 [Cargo 项目](/syntax/package_management)，不能单文件 `rustc`。

## 示例项目

示例位于 [`async_demo/`](https://github.com/Huauauaa/hi-rust/tree/main/async_demo)：

**`Cargo.toml`**：

```toml
[package]
name = "async_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["rt", "rt-multi-thread", "macros", "time"] }
```

**`src/main.rs`**：

<<< ../../async_demo/src/main.rs{rust}

## `async fn` 与 `.await`

```rust
async fn fetch_label(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} ready")
}

let msg = fetch_label("hi-rust", 10).await;
```

- `async fn` 返回值被包装为 `impl Future<Output = T>`。
- `.await` 只能在 `async` 上下文（`async fn` 内或 `async { ... }` 块）中使用。
- 遇到 `.await` 时，若 Future 未完成，当前任务**让出**执行权，运行时去跑其他任务。

## 运行时入口

Future 需要运行时驱动。Tokio 常用宏：

```rust
#[tokio::main]
async fn main() {
    // ...
}
```

等价于在同步 `main` 里创建运行时并 `block_on` 顶层 Future。其他运行时（如 `async-std`）有各自入口宏。

## 顺序与并发

**顺序** `.await`：前一个完成后再开始下一个，总耗时约为各段之和。

```rust
let a = fetch_label("A", 50).await;
let b = fetch_label("B", 50).await;
```

**并发**执行多个 Future：用 `tokio::join!` 同时 poll，总耗时接近最慢的一个：

```rust
let (a, b) = tokio::join!(
    fetch_label("A", 50),
    fetch_label("B", 50),
);
```

也可用 `tokio::spawn` 把任务丢到运行时后台：

```rust
let handle = tokio::spawn(async { 42 });
let n = handle.await.unwrap();
```

`spawn` 常配合 `move` 闭包，见 [闭包](/syntax/closures) 与 [多线程](/syntax/threads) 中的所有权说明。

## 与同步 I/O 的关系

标准库 `std::fs`、`std::thread::sleep` 等是**阻塞**的，在 async 任务里直接调用会占住线程。生产代码常用 `tokio::fs`、`tokio::time::sleep` 等异步 API，或通过 `spawn_blocking` 把阻塞工作交给专用线程池。

入门可先理解 `.await` 语义；完整异步 I/O 需结合具体 crate 文档。

## 运行

```sh
cd async_demo
cargo run
```

首次运行会下载 Tokio 依赖，需联网。

预期输出（并发段耗时约 50–60 ms，因机器略有差异）：

```text
=== async fn 与 .await ===
hi-rust ready
=== 顺序 .await ===
sequential: A ready, B ready
=== 并发 tokio::join! ===
concurrent: A ready, B ready in 56 ms
=== tokio::spawn ===
spawn result = 42
```

## 小结

| 概念 | 说明 |
|------|------|
| `async fn` | 返回 Future，不立即执行 |
| `.await` | 等待 Future 完成，未完成时让出任务 |
| 运行时 | Tokio 等负责调度 Future |
| `#[tokio::main]` | 异步程序入口 |
| `join!` / `spawn` | 并发多个异步任务 |
| Cargo + 依赖 | 实际 async 项目标配 |

## 延伸阅读

- [多线程](/syntax/threads) — OS 线程与 `Arc<Mutex<T>>`
- [IO](/syntax/io) — 同步标准 I/O
- [包管理](/syntax/package_management) — Cargo 与依赖
- [The Rust Book — Async](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [Tokio 教程](https://tokio.rs/tokio/tutorial)
