# 闭包

**闭包**（closure）是可以捕获周围变量的匿名函数。写法是 `|参数| 表达式` 或 `|参数| { 语句 }`，常传给迭代器、线程或高阶函数。

## 示例代码

源码在仓库根目录的 [`closures.rs`](https://github.com/Huauauaa/hi-rust/blob/main/closures.rs)：

<<< ../../closures.rs{rust}

## 基本语法

```rust
let add_one = |x| x + 1;
let multiply = |a, b| a * b;

add_one(5);      // 6
multiply(3, 4);  // 12
```

- 参数类型通常由用法**推断**；推断失败时需手动标注，如 `|x: i32| x + 1`。
- 单表达式可省略 `{}` 和 `return`；多语句时用花括号，返回值表达式不加 `;`。

## 捕获环境

闭包可以使用定义位置可见的变量：

```rust
let factor = 10;
let scale = |n| n * factor;
scale(7);  // 70
```

编译器按使用方式决定捕获方式是借用、可变借用还是取得所有权。

## `move` 关键字

`move` 强制闭包**取得**所捕获变量的所有权，常用于把闭包交给线程或延长生命周期（见 [多线程](/syntax/threads)）：

```rust
let s = String::from("hi-rust");
let greet = move || println!("Hello, {s}!");
greet();
// s 已移入闭包，此处不能再使用 s
```

## 作为参数传递

函数参数可写 `impl Fn` / `impl FnMut` / `impl FnOnce`，表示接受闭包：

```rust
fn apply_filter(nums: &[i32], mut pred: impl FnMut(i32) -> bool) -> Vec<i32> {
    nums.iter().copied().filter(|n| pred(*n)).collect()
}

apply_filter(&nums, |n| n % 2 == 0);
```

## 三种 trait

| Trait | 说明 |
|-------|------|
| `Fn` | 只不可变借用捕获变量，可多次调用 |
| `FnMut` | 可变借用捕获变量，可多次调用 |
| `FnOnce` | 取得捕获变量所有权，**最多调用一次** |

```rust
let mut count = 0;
let mut bump = || { count += 1; count };  // FnMut

let name = String::from("Rust");
let take_name = move || name;             // FnOnce
let owned = take_name();
```

参数类型选最宽松且够用的即可：`Fn` < `FnMut` < `FnOnce`（从约束少到多）。

## 与迭代器

`map`、`filter`、`fold` 等适配器的参数都是闭包：

```rust
let doubled: Vec<_> = nums.iter().map(|&n| n * 2).collect();
```

详见 [迭代器](/syntax/iterators)；更多实战用法见 [闭包的使用](/syntax/closure_usage)。

## 运行

在项目根目录执行：

```sh
rustc closures.rs
./closures
```

Windows 下运行 `closures.exe`。

预期输出：

```text
=== 基本语法 ===
add_one(5) = 6
multiply(3, 4) = 12
=== 捕获环境 ===
scale(7) = 70
=== move 闭包 ===
Hello, hi-rust!
=== 作为参数 ===
evens = [2, 4]
=== FnMut ===
bump = 1
bump = 2
=== FnOnce ===
owned = Rust
=== 与迭代器 ===
doubled = [2, 4, 6, 8, 10]
=== 高阶用法 ===
tick
tick
```

## 小结

| 写法 | 说明 |
|------|------|
| 闭包字面量 | 管道内写参数，后跟表达式或块 |
| 捕获外部变量 | 自动借用或移动 |
| `move` 闭包 | 强制取得捕获变量所有权 |
| `impl Fn` / `FnMut` / `FnOnce` | 接受闭包的参数类型 |
| 传给 `map` / `filter` 等 | 与迭代器配合 |

## 延伸阅读

- [`函数`](/syntax/functions) — 具名函数与 `fn`
- [`迭代器`](/syntax/iterators) — 闭包最常见的使用场景
- [`所有权和移动`](/syntax/ownership) — `move` 闭包的含义
- [The Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
