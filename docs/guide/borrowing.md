# 借用

**借用**（borrowing）允许在不取得所有权的情况下访问数据。用 `&T` 创建不可变引用（共享只读），用 `&mut T` 创建可变引用（独占可写）。所有者保留所有权，引用离开作用域后仍可继续使用原值。

## 示例代码

源码在仓库根目录的 [`borrowing.rs`](https://github.com/Huauauaa/hi-rust/blob/main/borrowing.rs)：

<<< ../../borrowing.rs{rust}

## 不可变借用 `&T`

`&` 表示借用，不转移所有权：

```rust
let text = String::from("hi-rust");
let n = len(&text);
println!("{text}"); // 仍有效
```

函数参数写成 `&String` 或更通用的 `&str`，调用时传 `&变量` 或字面量（自动借用）：

```rust
fn len(s: &String) -> usize { s.len() }
fn first_word(s: &str) -> &str { ... }
```

`&str` 是字符串切片引用，可接受 `&String` 和字符串字面量。

## 多个不可变借用

同一时刻可以有**任意多个**不可变引用，只要都不修改数据：

```rust
let r1 = &x;
let r2 = &x;
```

## 可变借用 `&mut T`

需要修改借来的值时，用 `&mut`：

```rust
let mut msg = String::from("Rust");
append_exclaim(&mut msg);
msg.push_str(" 🦀");
```

可变引用**同一时刻只能有一个**，且不能与不可变引用共存——编译器在编译期检查，避免数据竞争。

## 借用规则

1. 任意时刻，**要么**一个 `&mut`，**要么**任意多个 `&`（不能同时存在）。
2. 引用必须始终有效（不能指向已释放的值）。

违反规则时编译器会报错，而不是在运行时出现悬垂指针或未定义行为。

## 切片引用

对数组、字符串取子范围时，得到的是引用类型的**切片**：

```rust
let data = [1, 2, 3, 4, 5];
let slice = &data[1..4]; // 类型 &[i32]
```

切片不拥有数据，只借用一段连续元素。

## 运行

在项目根目录执行：

```sh
rustc borrowing.rs
./borrowing
```

Windows 下运行 `borrowing.exe`。

预期输出：

```text
=== 不可变借用 &T ===
len = 7, text 仍可用: hi-rust
first word: hello, s: hello world
=== 多个不可变借用 ===
r1 = 10, r2 = 10, x = 10
=== 可变借用 &mut T ===
msg = Rust!
msg = Rust! 🦀
=== 切片引用 ===
slice = [2, 3, 4]
```

## 小结

| 写法 | 说明 |
|------|------|
| `&T` | 不可变借用，可读不可写 |
| `&mut T` | 可变借用，独占可写 |
| `&str`、`&[T]` | 切片引用，借用一段数据 |
| 借用规则 | 一个 `&mut` 或多个 `&`，且引用必须有效 |

## 延伸阅读

- [`所有权和移动`](/guide/ownership) — Move 与 `clone`
- [`字符串`](/guide/strings) — `String` 与 `&str`
- [`内存`](/guide/memory) — 栈与堆（视频讲解）
