# 枚举

**枚举**（enum）定义一组可能的**变体**（variant），同一类型在任意时刻只能是其中一种。Rust 的枚举可以像结构体一样，为每个变体附带不同类型的数据，并与 `match`、`if let` 配合做模式匹配。

## 示例代码

源码在仓库根目录的 [`enums.rs`](https://github.com/Huauauaa/hi-rust/blob/main/enums.rs)：

<<< ../../enums.rs{rust}

## 简单枚举

只有变体名、不带额外数据：

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
```

用 `EnumName::Variant` 访问变体。同一枚举的所有变体属于同一类型。

## 带数据的变体

每个变体可以携带不同形状的数据：

```rust
enum Message {
    Quit,                        // 无数据
    Move { x: i32, y: i32 },     // 命名字段，类似 struct
    Write(String),               // 元组形式
    ChangeColor(u8, u8, u8),
}
```

- **单元变体** — 只有名字，如 `Quit`
- **结构体变体** — 花括号字段，如 `Move { x, y }`
- **元组变体** — 圆括号类型列表，如 `Write(String)`

这为「同一概念、多种形式」建模提供了类型安全的方式。

## match 匹配

处理枚举时，用 `match` 穷举各变体：

```rust
match kind {
    IpAddrKind::V4 => println!("IPv4"),
    IpAddrKind::V6 => println!("IPv6"),
}
```

带数据的变体可以在分支里解构：

```rust
match msg {
    Message::Write(text) => println!("{text}"),
    Message::Move { x, y } => println!("({x}, {y})"),
    Message::Quit => {}
    Message::ChangeColor(r, g, b) => { ... }
}
```

必须覆盖所有变体，或用 `_` 兜底。详见 [match 匹配](/syntax/match_patterns)。

## if let

只关心某一种变体时，用 `if let` 更简洁：

```rust
if let Message::Move { x, y } = mv {
    println!("move to ({x}, {y})");
}
```

等价于只写一个分支的 `match`。更多用法见 [if let 和 while let](/syntax/if_while_let)。

## impl 与方法

枚举也可以有 `impl` 块，定义方法：

```rust
impl Message {
    fn describe(&self) -> &str {
        match self {
            Message::Quit => "quit",
            Message::Move { .. } => "move",
            // ...
        }
    }
}
```

## Option：标准库枚举

`Option<T>` 是标准库中最常用的枚举，表示「有值」或「无值」：

```rust
enum Option<T> {
    Some(T),
    None,
}

let some_num = Some(5);
let no_num: Option<i32> = None;
```

比空指针更安全：必须显式处理 `None`，编译器会检查是否遗漏。更多用法见 [错误处理](/syntax/error_handling)。

## 运行

在项目根目录执行：

```sh
rustc enums.rs
./enums
```

Windows 下运行 `enums.exe`。

预期输出：

```text
=== 简单枚举 ===
route: IPv4
route: IPv6
=== 带数据的变体 ===
describe: write
describe: change_color
move to (10, 20)
=== match 枚举 ===
action: 退出
=== Option ===
some_num = 5
no_num is None
```

## 小结

| 写法 | 说明 |
|------|------|
| `enum Name { A, B }` | 简单变体 |
| `Variant(T)` | 元组变体 |
| `Variant { field: T }` | 结构体变体 |
| `Enum::Variant` | 构造枚举值 |
| `match` | 穷举处理各变体 |
| `if let` | 只匹配一种变体 |
| `Option<T>` | 有值 / 无值 |

## 延伸阅读

- [`结构体`](/syntax/structs) — 命名字段 vs 枚举变体
- [match 匹配](/syntax/match_patterns) — 模式、守卫与 `if let`
- [`条件判断`](/syntax/conditionals) — `if` 与 `match` 入门
- [The Rust Book — Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
