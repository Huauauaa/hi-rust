# Debug

**`Debug`** 是标准库中的 trait，用于**开发者向**的格式化输出——调试、日志、`println!` 里快速查看数据结构。习惯上用 `{:?}` 或 `{:#?}` 格式化；自定义 struct / enum 通常加 `#[derive(Debug)]` 自动实现。

面向用户的可读输出见 [`Display`](/syntax/display) trait 和 `{}`；本章只讲 `Debug`。需要手写实现、隐藏敏感字段等场景见 [自定义 Debug](/syntax/debug_custom)。

## 示例代码

源码在仓库根目录的 [`debug.rs`](https://github.com/Huauauaa/hi-rust/blob/main/debug.rs)：

<<< ../../debug.rs{rust}

## `{:?}` 调试格式

`println!`、`format!` 等宏用 `:?` 按 Debug 格式输出：

```rust
let n = 42;
println!("{:?}", n);   // 42
println!("{n:?}");     // 同上，更简洁
```

标准库中大多数类型已实现 `Debug`：`i32`、`bool`、`String`、`Vec`、`Option`、`Result` 等。

## `#[derive(Debug)]`

自定义类型默认**不能**用 `{:?}` 打印，需实现 `Debug`。最常见写法是派生：

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Banned(String),
}

let user = User { name: String::from("Alice"), age: 30 };
println!("{user:?}");  // User { name: "Alice", age: 30 }
```

`derive` 会为所有字段递归生成实现；任一字段未实现 `Debug`，则无法派生。`#[derive(Debug)]` 是**过程宏**，见 [函数与宏的区别](/qa/function_vs_macro)。

## `{:#?}` 美化输出

`#?` 在多行、带缩进的形式下打印，适合嵌套 struct / enum：

```rust
let status = Status::Banned(String::from("spam"));
println!("{status:#?}");
// Banned(
//     "spam",
// )
```

## `dbg!` 宏

`dbg!` 在**打印表达式值**的同时返回该值，便于临时调试：

```rust
let x = 42;
let y = dbg!(x * 2);  // [src/main.rs:10] x * 2 = 84
```

输出包含文件名与行号。调试完记得删除，不要留在正式代码里。

## Debug 与 Display

| | `Debug` (`:?`) | `Display` (`{}`) |
|--|----------------|------------------|
| 用途 | 开发调试 | 面向用户的可读输出 |
| 实现 | 常 `#[derive(Debug)]` | 通常手写 `impl Display` |
| 标准库 | 大多数类型已实现 | 仅部分类型（如 `String`、`&str`） |

需要自定义 `Display` 时，一般仍建议同时 `derive(Debug)`，便于测试与错误排查。详见 [`Display`](/syntax/display)。

## 运行

在项目根目录执行：

```sh
rustc debug.rs
./debug
```

Windows 下运行 `debug.exe`。

预期输出：

```text
=== derive Debug ===
User { name: "Alice", age: 30 }, name = Alice, age = 30
User { name: "Alice", age: 30 }
=== 美化输出 {:#?} ===
Active
Inactive
Banned(
    "spam",
)
reason = spam
=== dbg! 宏 ===
[debug.rs:34:13] x * 2 = 84
y = 84
=== 标准库类型 ===
Some(5), Err("oops")
=== 元组与 Vec ===
(1, "Rust"), [1, 2, 3]
```

## 小结

| 写法 | 说明 |
|------|------|
| `{:?}` / `{value:?}` | Debug 格式输出 |
| `{:#?}` | 多行美化 Debug 输出 |
| `#[derive(Debug)]` | 为 struct / enum 自动实现 Debug |
| `dbg!(expr)` | 打印并返回表达式的值 |
| `Debug` vs `Display` | 调试 vs 用户可读展示 |

## 延伸阅读

- [自定义 Debug](/syntax/debug_custom) — 手写 `impl Debug`
- [`Display`](/syntax/display) — `{}` 与用户可读输出
- [`结构体`](/syntax/structs) — 定义自定义类型
- [`枚举`](/syntax/enums) — enum 变体与派生 trait
- [`错误处理`](/syntax/error_handling) — `Result` 的 `{:?}` 输出
- [函数与宏的区别](/qa/function_vs_macro) — `derive` 过程宏
- [The Rust Book — Debug Trait](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
