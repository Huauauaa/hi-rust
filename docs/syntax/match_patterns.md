# match 匹配

`match` 是 Rust 的**模式匹配**表达式：把值与若干**模式**比对，命中第一个分支后执行对应代码。编译器要求匹配**穷尽**所有可能；未覆盖时会报错。`if` 分支见 [条件判断](/syntax/conditionals)。

## 示例代码

源码在仓库根目录的 [`match_patterns.rs`](https://github.com/Huauauaa/hi-rust/blob/main/match_patterns.rs)：

<<< ../../match_patterns.rs{rust}

## 基本语法

```rust
let weekday = match day {
    1 => "周一",
    2 => "周二",
    _ => "其他",
};
```

- 每个分支：`模式 => 表达式或块`。
- 整体是表达式，各分支返回值类型须一致。
- 必须覆盖所有情况，或用 `_` 通配兜底。

## 常用模式

| 模式 | 说明 | 示例 |
|------|------|------|
| 字面量 | 精确匹配 | `1`、`"ok"` |
| `\|` | 多个模式同一分支 | `2 \| 3 => ...` |
| `..=` | 闭区间（整数/字符） | `90..=100` |
| `_` | 匹配任意，不绑定 | `_ => ...` |
| `@` | 绑定整个值并解构 | `n @ 1..=5 => ...` |

```rust
match score {
    n @ 90..=100 => format!("A ({n})"),
    80..=89 => String::from("B"),
    _ => String::from("C"),
}
```

## 解构 enum 与 struct

`match` 可一次性拆出 enum 变体或 struct 字段：

```rust
match msg {
    Message::Quit => { ... }
    Message::Move { x, y } => { ... }
    Message::Write(text) => { ... }
    Message::ChangeColor(r, g, b) => { ... }
}

match point {
    Point { x: 0, y } => { ... }
    Point { x, y } => { ... }
}
```

元组同样可解构：`match (x, y) { (0, y) => ..., _ => ... }`。enum 定义见 [枚举](/syntax/enums)，struct 见 [结构体](/syntax/structs)。

## match 守卫（guard）

模式后可加 `if` 条件，进一步过滤：

```rust
match num {
    Some(x) if x % 2 == 0 => println!("even {x}"),
    Some(x) => println!("odd {x}"),
    None => println!("none"),
}
```

守卫不改变穷尽性检查：仍须覆盖 `None`、`Some` 等所有变体。

## `if let` 与 `while let`

只关心一种模式时，不必写完整 `match`。详见 [if let 和 while let](/syntax/if_while_let)。

```rust
if let Message::Write(text) = msg {
    println!("{text}");
}

while let Some(n) = stack.pop() {
    println!("{n}");
}
```

## `ref` 与 `ref mut`

模式中写 `ref` 绑定**引用**，避免移动原值：

```rust
match value {
    ref r => println!("{r}"),  // value 仍可用
}
```

可变匹配用 `ref mut`，详见 [变量绑定](/syntax/bindings)。

## 运行

在项目根目录执行：

```sh
rustc match_patterns.rs
./match_patterns
```

Windows 下运行 `match_patterns.exe`。

预期输出：

```text
=== 字面量与 _ ===
n = 3 → two or three
=== 范围与 @ 绑定 ===
score 95 → A (95)
=== 解构 enum ===
move to (10, 20)
rgb = (255, 128, 0)
=== 解构 struct 与元组 ===
point (1, 2)
pair y = 5
=== match 守卫 ===
even 4
=== if let ===
if let: hi-rust
=== ref 匹配 ===
ref = 42, value still = 42
```

## 小结

| 写法 | 说明 |
|------|------|
| `match val { pat => ... }` | 穷尽模式匹配 |
| `_` | 通配兜底 |
| `a \| b` | 多模式合并 |
| `m..=n` | 范围模式 |
| `x @ pat` | 绑定并匹配 |
| `pat if cond` | match 守卫 |
| `if let` / `while let` | 单模式简写 |

## 延伸阅读

- [条件判断](/syntax/conditionals) — `if` 与 `match` 入门
- [枚举](/syntax/enums) — 用 `match` 处理变体
- [变量绑定](/syntax/bindings) — 模式解构与 `ref`
- [错误处理](/syntax/error_handling) — `match` 处理 `Result`
- [The Rust Book — Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
