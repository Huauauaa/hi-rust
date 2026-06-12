# 生命周期

**生命周期**（lifetime）描述引用**有效的作用域**——确保引用不会比它指向的数据活得更久，从而避免悬垂引用。Rust 在编译期检查生命周期，多数情况下编译器能自动推断；无法推断时，需要显式标注 `'a` 等生命周期参数。

基础借用规则见 [借用](/syntax/borrowing)；本章聚焦「引用能活多久」以及如何在类型签名里表达这种关系。

## 示例代码

源码在仓库根目录的 [`lifetimes.rs`](https://github.com/Huauauaa/hi-rust/blob/main/lifetimes.rs)：

<<< ../../lifetimes.rs{rust}

## 为什么需要生命周期

引用必须始终指向有效数据。若函数返回一个引用，编译器需要知道返回值与哪些输入参数「绑在一起」：

```rust
// 编译错误：无法判断返回的引用来自 x 还是 y
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

加上生命周期标注后，含义是：**返回值的生命周期不超过 `x` 和 `y` 中较短的那个**：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

- `'a` 是生命周期参数名，惯例从 `'a`、`'b` 起。
- 读作：`x`、`y`、`返回值` 共享同一生命周期 `'a`。

## 函数签名中的 `'a`

泛型函数在 `<T>` 里声明类型参数；带引用的函数在 `<'a>` 里声明生命周期参数：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

调用时不必手动传入 `'a`，编译器根据实参推断。

## struct 中的生命周期

struct 里若包含引用字段，必须为 struct 声明生命周期参数：

```rust
struct Excerpt<'a> {
    part: &'a str,
}
```

表示 `Excerpt` 实例不能比其 `part` 指向的字符串活得更久。创建实例时，编译器会检查引用来源是否足够长：

```rust
let novel = String::from("Call me Ishmael...");
let first = novel.split('.').next().unwrap();
let excerpt = Excerpt { part: first };
// novel 必须比 excerpt 活得更久
```

为带生命周期的 struct 实现方法时，`impl` 也要写相同参数：

```rust
impl<'a> Excerpt<'a> {
    fn announce(&self) { ... }
}
```

## 生命周期省略

许多常见写法**不必**手写 `'a`，编译器按**省略规则**自动补全，例如：

- 每个引用参数各有一个独立生命周期
- 若只有一个输入生命周期，赋给所有输出
- 若方法是 `&self` / `&mut self`，输出生命周期与 `self` 相同

因此日常写 `fn len(s: &str) -> usize` 或 `fn first_word(s: &str) -> &str` 时通常不用显式标注；只有编译器报「missing lifetime specifier」时才需要补上。

## `'static` 生命周期

`'static` 表示引用在整个程序运行期间都有效，例如字符串字面量：

```rust
let s: &'static str = "hello";
```

`'static` 也可作为 trait 约束（如 `T: 'static`），表示类型不含非 `'static` 的引用——见 [多线程](/syntax/threads) 等进阶场景。

## 运行

在项目根目录执行：

```sh
rustc lifetimes.rs
./lifetimes
```

Windows 下运行 `lifetimes.exe`。

预期输出：

```text
=== 函数中的生命周期 ===
longest = long string
=== struct 中的生命周期 ===
excerpt: Call me Ishmael
novel still valid: Call me Ishmael. Som...
=== 'static ===
I live for the entire program
```

## 小结

| 写法 | 说明 |
|------|------|
| `&'a T` | 引用有效范围受 `'a` 约束 |
| `fn f<'a>(x: &'a str) -> &'a str` | 返回值与参数共享生命周期 |
| `struct S<'a> { r: &'a T }` | struct 不能比 `r` 指向的数据活得更久 |
| 生命周期省略 | 常见模式由编译器自动推断 |
| `'static` | 整个程序期间有效 |

## 延伸阅读

- [`借用`](/syntax/borrowing) — `&T` / `&mut T` 与借用规则
- [`所有权和移动`](/syntax/ownership) — 值何时被 drop
- [`泛型`](/syntax/generics) — 类型参数与 trait 约束
- [`智能指针`](/syntax/smart_pointers) — 需要共享所有权时可绕过部分生命周期限制
- [The Rust Book — Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
