# 所有权和移动

Rust 通过**所有权**（ownership）在编译期管理内存：每个值有且仅有一个所有者；所有者离开作用域时，值会被自动释放（`drop`）。把堆上数据赋给新变量时，所有权**移动**（move）过去，原变量不再有效。

## 示例代码

源码在仓库根目录的 [`ownership.rs`](https://github.com/Huauauaa/hi-rust/blob/main/ownership.rs)：

<<< ../../ownership.rs{rust}

## 所有权规则

1. 每个值有且仅有一个所有者。
2. 同一时刻只能有一个所有者。
3. 所有者离开作用域，值被 `drop`。

## Copy 与 Move

实现了 `Copy` trait 的类型（如 `i32`、`bool`、`char`）在赋值时**复制**栈上的值，原变量仍可用：

```rust
let a = 5;
let b = a;
println!("{a}"); // 仍有效
```

`String`、`Vec` 等堆上分配的类型在赋值时**移动**所有权，原变量失效：

```rust
let s1 = String::from("hello");
let s2 = s1;       // 所有权移给 s2
// println!("{s1}"); // 编译错误：value borrowed after move
println!("{s2}");
```

移动只复制栈上的指针、长度、容量；堆上数据不会被深拷贝，因此高效且安全。

## 作用域与 drop

变量在 `{}` 块或函数结束时离开作用域，Rust 自动调用 `drop` 释放资源：

```rust
{
    let temp = String::from("scope");
    println!("{temp}");
} // temp 在此 drop
```

无需手动 `free`，也不会泄漏。

## 函数与所有权

参数按值传入时，所有权移入函数；函数结束时参数被 `drop`：

```rust
fn take_ownership(s: String) {
    println!("{s}");
} // s drop

let s3 = String::from("transfer");
take_ownership(s3); // s3 不再有效
```

函数也可以**返回**所有权，把值交给调用方：

```rust
fn give_ownership() -> String {
    String::from("returned")
}

let s4 = give_ownership();
```

`i32` 等 `Copy` 类型传给函数时仍是复制，调用后原变量照常使用。

## clone

需要保留原值又要得到一份独立副本时，显式调用 `.clone()`：

```rust
let original = String::from("hi-rust");
let cloned = original.clone();
println!("{original}, {cloned}"); // 两者都有效
```

`clone` 会深拷贝堆数据，有运行时开销；只在确实需要两份所有权时使用。

## 运行

在项目根目录执行：

```sh
rustc ownership.rs
./ownership
```

Windows 下运行 `ownership.exe`。

预期输出：

```text
=== Copy 类型 ===
a = 5, b = 5
=== Move：String ===
s2 = hello
=== 作用域与 drop ===
temp = scope
temp 已离开作用域
=== 函数与所有权 ===
take_ownership: transfer
s4 = returned
copy_integer: 42
=== clone ===
original = hi-rust, cloned = hi-rust
```

## 小结

| 概念 | 说明 |
|------|------|
| 所有权 | 每个值唯一所有者，离开作用域自动释放 |
| Move | 堆类型赋值/传参时转移所有权，原变量失效 |
| Copy | 栈上简单类型赋值时复制，双方仍有效 |
| `drop` | 作用域结束时自动释放资源 |
| `.clone()` | 深拷贝，得到独立副本 |

## 延伸阅读

- [`借用`](/syntax/borrowing) — 在不取得所有权的情况下使用值
- [`智能指针`](/syntax/smart_pointers) — `Box`、`Rc` 等堆上所有权扩展
- [`字符串`](/syntax/strings) — `String` 与 `&str` 的区别
- [`内存`](/qa/memory) — 栈与堆、借用（视频讲解）
