# 模块

**模块**（module）组织代码、控制**隐私**并管理命名空间。crate 根是 `main.rs` 或 `lib.rs`；子模块可写在同一文件内，也可拆成独立文件。多文件模块需配合 [Cargo 项目](/syntax/package_management) 使用。

## 示例项目

示例位于 [`modules_demo/`](https://github.com/Huauauaa/hi-rust/tree/main/modules_demo)：

```text
modules_demo/
├── Cargo.toml
└── src/
    ├── main.rs           # crate 根
    ├── garden.rs         # mod garden;
    ├── garden/
    │   └── vegetables.rs # garden 的子模块
    └── utils.rs
```

**`src/main.rs`** — 声明模块并 `use`：

<<< ../../modules_demo/src/main.rs{rust}

**`src/garden.rs`** — 子模块与 `pub use` 重导出：

<<< ../../modules_demo/src/garden.rs{rust}

**`src/garden/vegetables.rs`** — 嵌套模块与隐私：

<<< ../../modules_demo/src/garden/vegetables.rs{rust}

**`src/utils.rs`** — 私有函数与公开 API：

<<< ../../modules_demo/src/utils.rs{rust}

## 声明模块

| 写法 | 说明 |
|------|------|
| `mod garden;` | 从 `garden.rs` 或 `garden/mod.rs` 加载 |
| `mod inline { ... }` | 在同一文件内定义 |
| `pub mod garden;` | 允许外部 crate 访问该模块 |

`main.rs` 中的 `mod garden;` 对应 `src/garden.rs`；`garden.rs` 里的 `pub mod vegetables;` 对应 `src/garden/vegetables.rs`。

## 路径

| 前缀 | 含义 |
|------|------|
| `crate::` | 当前 crate 根 |
| `super::` | 父模块 |
| `self::` | 当前模块 |
| `garden::vegetables::tomato` | 从 crate 根起的绝对路径 |

## `use` 与重导出

```rust
use garden::Plant;              // 引入名称到当前作用域
use utils::greet;               // 调用时可写 greet(...) 而非 utils::greet(...)

pub use vegetables::Plant;      // 在 garden 层重导出，外部可用 garden::Plant
```

`use` 只影响名称书写，不改变隐私；未标记 `pub` 的项只在模块内部及子模块可见。

## 隐私规则

- 默认**私有**：函数、结构体、字段等不加 `pub` 则仅本模块及子模块可访问。
- `pub` 对外公开；`pub(crate)` 仅 crate 内可见（未在示例中展开）。
- 子模块可调用同模块内的私有函数（如 `public_answer` 调用 `internal_helper`）。

## 同一文件内的模块

小示例也可用 inline 模块，无需额外文件：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn seat_guest() {
            println!("guest seated");
        }
    }
}

use front_of_house::hosting;

fn main() {
    hosting::seat_guest();
}
```

## 运行

```sh
cd modules_demo
cargo run
```

预期输出：

```text
=== 文件模块 ===
garden module
plant = tomato
label = tomato (ready to pick)
=== pub use 重导出 ===
plant = spinach
=== use 路径 ===
Hello, hi-rust
=== 隐私规则 ===
public = 42
```

## 小结

| 写法 | 说明 |
|------|------|
| `mod name;` | 加载文件模块 |
| `pub mod` / `pub fn` | 对外公开 |
| `use path::Item` | 引入名称 |
| `pub use` | 重导出 |
| `crate::` / `super::` | 路径前缀 |

## 延伸阅读

- [包管理](/syntax/package_management) — Cargo 与 `src/` 布局
- [`结构体`](/syntax/structs) — 模块内定义类型
- [命名规范](/spec/naming) — 模块与文件命名约定
- [The Rust Book — Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
