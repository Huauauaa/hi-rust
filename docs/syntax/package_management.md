# 包管理

Rust 官方包管理器和构建工具是 **Cargo**。它负责创建项目、拉取依赖、编译、运行和测试。本仓库根目录的单文件示例用 `rustc` 即可；实际项目几乎都用 Cargo。

## 示例项目

示例 Cargo 项目位于 [`cargo_demo/`](https://github.com/Huauauaa/hi-rust/tree/main/cargo_demo)：

**`Cargo.toml`** — 项目清单（manifest）：

```toml
[package]
name = "cargo_demo"
version = "0.1.0"
edition = "2021"
description = "hi-rust Cargo demo crate"

[dependencies]
```

**`src/lib.rs`** — 库 crate 根模块：

<<< ../../cargo_demo/src/lib.rs{rust}

**`src/main.rs`** — 二进制入口，依赖同包的库：

<<< ../../cargo_demo/src/main.rs{rust}

## 创建项目

```sh
cargo new my_app        # 可执行项目（含 src/main.rs）
cargo new my_lib --lib  # 库项目（含 src/lib.rs）
cargo init              # 在当前目录初始化
```

生成的标准布局：

```text
my_app/
├── Cargo.toml
├── src/
│   └── main.rs
└── .gitignore
```

编译产物在 `target/`，无需提交到 Git。

## 常用命令

| 命令 | 说明 |
|------|------|
| `cargo build` | 编译（默认 debug） |
| `cargo build --release` | 优化编译 |
| `cargo run` | 编译并运行二进制 |
| `cargo check` | 只检查，不产出可执行文件（更快） |
| `cargo test` | 运行 `#[test]` 与 `#[cfg(test)]` 模块 |
| `cargo doc --open` | 生成并打开文档 |

在示例目录中：

```sh
cd cargo_demo
cargo run
cargo test
```

## 添加依赖

在 `Cargo.toml` 的 `[dependencies]` 中声明：

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
```

也可用命令行添加（Cargo 1.62+）：

```sh
cargo add serde --features derive
```

Cargo 会从 [crates.io](https://crates.io/) 下载依赖，并在 `Cargo.lock` 中锁定具体版本（应用项目建议提交 lock 文件）。

## crate 类型

| 类型 | 入口 | 说明 |
|------|------|------|
| 二进制（bin） | `src/main.rs` | 可执行程序 |
| 库（lib） | `src/lib.rs` | 供其他 crate 引用 |
| 同一包可同时有 lib + bin | 两者并存 | 示例项目即此结构 |

二进制中通过包名引用库：`use cargo_demo::greet;`（包名来自 `Cargo.toml` 的 `name`）。模块与文件组织见 [模块](/syntax/modules)。

## 内置环境变量

构建时 Cargo 注入环境变量，例如：

| 变量 | 说明 |
|------|------|
| `CARGO_PKG_NAME` | 包名 |
| `CARGO_PKG_VERSION` | 版本号 |
| `CARGO_MANIFEST_DIR` | `Cargo.toml` 所在目录 |

示例中用 `env!("CARGO_PKG_VERSION")` 在编译期嵌入版本字符串。

## 与 `rustc` 的对比

| | `rustc hello.rs` | Cargo 项目 |
|--|------------------|------------|
| 依赖 | 手动管理 | `Cargo.toml` 声明 |
| 多文件 | 需手动指定 | `mod` + 目录自动参与编译 |
| 测试 | 自行组织 | `cargo test` 内置 |
| 适用 | 学习语法片段 | 实际开发与发布 |

本仓库语法章节的 `.rs` 文件便于复制运行；完整项目请用 `cargo new` 起步。

## 运行

```sh
cd cargo_demo
cargo run
```

Windows 下命令相同。预期输出：

```text
=== cargo run ===
Hello, hi-rust!
crate: cargo_demo v0.1.0
```

运行测试：

```sh
cargo test
```

预期看到 `greet_works` 测试通过。

## 小结

| 概念 | 说明 |
|------|------|
| `Cargo.toml` | 包名、版本、edition、依赖 |
| `cargo new` / `init` | 创建项目 |
| `cargo run` / `build` / `test` | 构建与测试 |
| `[dependencies]` | 外部 crate |
| `src/main.rs` / `src/lib.rs` | bin / lib 入口 |
| `Cargo.lock` | 依赖版本锁定 |

## 延伸阅读

- [`Hello, World`](/syntax/hello_world) — 单文件 `rustc` 入门
- [`错误处理`](/syntax/error_handling) — 库中常用的 `Result`
- [The Rust Book — Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Cargo 官方文档](https://doc.rust-lang.org/cargo/)
