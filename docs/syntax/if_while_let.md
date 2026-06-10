# if let 和 while let

当只关心**一种**模式时，不必写完整 `match`。`if let` 在匹配成功时执行分支；`while let` 在每次匹配成功时重复执行循环体。二者都是模式匹配的简写，语法基础见 [match 匹配](/syntax/match_patterns)。

## 示例代码

源码在仓库根目录的 [`if_while_let.rs`](https://github.com/Huauauaa/hi-rust/blob/main/if_while_let.rs)：

<<< ../../if_while_let.rs{rust}

## `if let`

```rust
if let Some(x) = optional {
    println!("{x}");
}
```

- 模式匹配成功 → 进入 `{}` 块，并可绑定变量。
- 匹配失败 → 跳过整个 `if let`（除非写了 `else`）。

### 与 `match` 的对比

```rust
// 等价写法
match optional {
    Some(x) => println!("{x}"),
    _ => {}
}
```

只处理一种情况时用 `if let` 更简洁；需穷尽多种分支时仍用 `match`。

### `else` 分支

```rust
if let Some(x) = none {
    println!("{x}");
} else {
    println!("none branch");
}
```

`if let` 也可作为**表达式**，两分支类型须一致：

```rust
let mode = if let Some(m) = config { m } else { "default" };
```

### 处理 enum

```rust
if let Event::Click { x, y } = event {
    println!("click at ({x}, {y})");
}
```

见 [枚举](/syntax/enums) 中的变体解构。

### 额外条件

`if let` 本身没有 `match` 那样的守卫语法；需要额外判断时嵌套 `if`：

```rust
if let Some(x) = num {
    if x % 2 == 0 {
        println!("even {x}");
    }
}
```

复杂条件可改用带守卫的 `match`。

## `while let`

```rust
while let Some(top) = stack.pop() {
    println!("{top}");
}
```

- 每次循环先尝试匹配；成功则执行循环体并再次匹配。
- 匹配失败则**退出循环**（不是 `break` 才退出，而是模式不匹配即结束）。

### 典型场景

| 场景 | 示例 |
|------|------|
| 栈/队列逐个弹出 | `while let Some(x) = vec.pop()` |
| 迭代器消费 | `while let Some(n) = iter.next()` |
| 重复直到 `None` | `Option` 链式处理 |

```rust
let mut nums = vec![10, 20, 30].into_iter();
while let Some(n) = nums.next() {
    print!("{n} ");
}
```

与 `for n in iter` 类似；`while let` 适合「每次循环还要对返回值做模式匹配」的场景。

## `let else`

Rust 1.65+ 支持 **let-else**：模式不匹配时执行 `else` 块并**必须从该块 diverge**（`return`、`break`、`panic!` 等）：

```rust
let Some(value) = optional else {
    return;
};
// 此处 value 已绑定
```

适合「必须是某种形状，否则提前退出」的写法，见 [错误处理](/syntax/error_handling) 中的 `Option` 处理。

## 运行

在项目根目录执行：

```sh
rustc if_while_let.rs
./if_while_let
```

Windows 下运行 `if_while_let.exe`。

预期输出：

```text
=== if let 与 Option ===
got 10
none branch
=== if let 与 enum ===
click at (42, 7)
quit event
=== if let 与额外条件 ===
even 4
=== if let else 表达式 ===
mode = prod
fallback = default
=== while let ===
pop 3 pop 2 pop 1 
6 5 4 
=== while let 与迭代器 ===
10 20 30 
=== let else ===
value = 99
```

## 小结

| 写法 | 说明 |
|------|------|
| `if let PAT = val { ... }` | 单模式匹配，失败则跳过 |
| `if let ... else { ... }` | 匹配失败走 else；作表达式时两分支同型 |
| `while let PAT = val { ... }` | 反复匹配，失败则结束循环 |
| `let PAT = val else { ... }` | 失败时必须从 else 发散 |

## 延伸阅读

- [match 匹配](/syntax/match_patterns) — 完整模式与守卫
- [枚举](/syntax/enums) — `if let` 解构变体
- [错误处理](/syntax/error_handling) — `Option` / `Result` 与 `if let`
- [循环](/syntax/loops) — `for`、`while`、`loop`
- [The Rust Book — if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)
