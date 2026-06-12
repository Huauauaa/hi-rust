# 自定义 Debug

`#[derive(Debug)]` 会打印**全部字段**，适合快速开发。需要控制输出格式、隐藏 token / 密码等敏感信息，或 enum 各变体展示方式不同时，应手写 **`impl Debug`**。

基础 `{:?}`、`derive` 用法见 [Debug](/syntax/debug)；本章只讲自定义实现。

## 示例代码

源码在仓库根目录的 [`debug_custom.rs`](https://github.com/Huauauaa/hi-rust/blob/main/debug_custom.rs)：

<<< ../../debug_custom.rs{rust}

## 为什么不用 derive

| | `#[derive(Debug)]` | 手写 `impl Debug` |
|--|-------------------|-------------------|
| 速度 | 一行搞定 | 需写 `fmt` 方法 |
| 输出 | 所有字段原样打印 | 完全可控 |
| 敏感字段 | 会泄露 | 可省略或替换 |
| `{:#?}` | 自动支持 | 用 helper 时同样支持 |

## struct：`debug_struct`

标准库提供 **`debug_struct`**，输出风格接近 `derive`，且自动支持 `{:#?}` 美化：

```rust
use std::fmt;

struct User {
    name: String,
    age: u32,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}

println!("{user:?}");  // User { name: "Alice", age: 30 }
```

- 第一个参数是类型名（字符串）。
- `.field("字段名", &值)` 逐个添加字段。
- `.finish()` 完成输出。

## enum：按变体匹配

enum 在 `match` 里分别处理各变体：

```rust
impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => f.write_str("Active"),
            Status::Inactive => f.write_str("Inactive"),
            Status::Banned(reason) => f.debug_tuple("Banned").field(reason).finish(),
        }
    }
}
```

- 无字段变体用 `f.write_str` 或 `write!`。
- 有字段变体用 **`debug_tuple`**（元组风格）或 **`debug_struct`**（结构体风格）。

## 隐藏敏感字段

`derive` 会把 `token` 原样打进日志；手写时可替换为占位符：

```rust
struct Account {
    username: String,
    token: String,
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Account")
            .field("username", &self.username)
            .field("token", &"<redacted>")
            .finish()
    }
}
// Account { username: "alice", token: "<redacted>" }
```

也可直接**不写**某个 `.field(...)`，该字段就不会出现在 Debug 输出里。

## 其他 helper

| 方法 | 用途 |
|------|------|
| `f.debug_struct("Name")` | struct 风格 |
| `f.debug_tuple("Name")` | 元组 / enum 变体风格 |
| `f.debug_list()` | 列表风格（如 `[1, 2, 3]`） |
| `f.debug_set()` | 集合风格 |

用 `write!` / `f.write_str` 可完全自定义格式，但 `{:#?}` 的缩进需自己处理；一般优先用上述 helper。

## 运行

在项目根目录执行：

```sh
rustc debug_custom.rs
./debug_custom
```

Windows 下运行 `debug_custom.exe`。

预期输出：

```text
=== struct Debug ===
User { name: "Alice", age: 30 }
=== enum Debug ===
Active
Inactive
Banned("spam")
reason = spam
=== 隐藏敏感字段 ===
Account { username: "alice", token: "<redacted>" }, token len = 16
=== 美化输出 {:#?} ===
User {
    name: "Alice",
    age: 30,
}
Banned(
    "spam",
)
```

## 小结

| 写法 | 说明 |
|------|------|
| `impl fmt::Debug for T` | 手写 Debug 实现 |
| `debug_struct` / `debug_tuple` | 结构化输出的 helper |
| `.field(name, &val)` | 添加命名字段 |
| 省略 `.field` | 不输出该字段 |
| `&"<redacted>"` | 替换敏感值 |

## 延伸阅读

- [Debug](/syntax/debug) — `{:?}` 与 `#[derive(Debug)]`
- [`Display`](/syntax/display) — 用户可读输出，同样需手写
- [`结构体`](/syntax/structs) — 自定义类型
- [`枚举`](/syntax/enums) — enum 变体
