# 闭包的使用

闭包语法与 `Fn` trait 见 [闭包](/syntax/closures)。本章聚焦**实际用法**：迭代器链、排序、`Option`/`Result` 组合、自定义高阶函数与闭包工厂等日常场景。

## 示例代码

源码在仓库根目录的 [`closure_usage.rs`](https://github.com/Huauauaa/hi-rust/blob/main/closure_usage.rs)：

<<< ../../closure_usage.rs{rust}

## 迭代器链

`map`、`filter`、`reduce` 等适配器的参数都是闭包，可链式组合：

```rust
let evens: Vec<_> = nums
    .iter()
    .filter(|&&n| n % 2 == 0)
    .map(|&n| n * 10)
    .collect();

let sum: i32 = nums.iter().sum();
let max = nums.iter().copied().reduce(|a, b| a.max(b));
```

闭包可**捕获**外部变量，无需把阈值等再传进迭代器 API：

```rust
let threshold = 5;
let above: Vec<_> = nums.iter().copied().filter(|n| *n > threshold).collect();
```

详见 [迭代器](/syntax/iterators)。

## 排序与比较

`slice::sort_by` 用闭包定义元素间顺序：

```rust
words.sort_by(|a, b| a.len().cmp(&b.len()));
```

按字段排序可用 `sort_by_key`：`.sort_by_key(|w| w.len())`。

## `Option` 与 `Result`

标准库许多方法接受闭包，避免嵌套 `match`：

| 方法 | 说明 |
|------|------|
| `.map(f)` | 有值时变换内容 |
| `.and_then(f)` | 有值时链式返回新 `Option` |
| `.unwrap_or_else(f)` | 失败/无值时用闭包生成默认值 |
| `.map_err(f)` | 变换 `Result` 的错误 |

```rust
let doubled = "42".parse::<i32>().ok().map(|n| n * 2);

let value = None::<i32>.unwrap_or_else(|| 100);

let n = "oops".parse::<i32>().unwrap_or_else(|e| {
    println!("{e}");
    0
});
```

见 [错误处理](/syntax/error_handling)。

## 自定义高阶函数

把闭包作为参数，封装可复用逻辑：

```rust
fn pick_if(nums: &[i32], min: i32, test: impl Fn(i32) -> bool) -> Vec<i32> {
    nums.iter().copied().filter(|n| *n >= min && test(*n)).collect()
}

pick_if(&nums, 4, |n| n % 2 == 0);
```

需要多次调用传入的闭包时，参数用 `impl FnMut()` 而非 `impl Fn()`。

## 闭包工厂

函数可返回闭包（常配合 `move`），用于生成一组相关行为：

```rust
fn make_adder(base: i32) -> impl Fn(i32) -> i32 {
    move |x| base + x
}

let add5 = make_adder(5);
add5(7);  // 12
```

每次调用 `make_adder` 会生成捕获不同 `base` 的新闭包。

## `inspect`：链中副作用

想在迭代过程中打印或调试，又不改变元素，用 `inspect`：

```rust
nums.iter()
    .copied()
    .inspect(|n| { if *n > 5 { print!("[{n}] "); } })
    .collect::<Vec<_>>();
```

`inspect` 返回原样元素，仅借闭包做观察；复杂变换仍用 `map`。

## 其他常见场景

| 场景 | 闭包用法 |
|------|----------|
| 多线程 | `thread::spawn(move \|\| { ... })`，见 [多线程](/syntax/threads) |
| 延迟计算 | `unwrap_or_else(\|\| expensive())` 仅在需要时执行 |
| 回调 | GUI / 异步中传递 `\|\| ...`（进阶 crate） |

## 运行

在项目根目录执行：

```sh
rustc closure_usage.rs
./closure_usage
```

Windows 下运行 `closure_usage.exe`。

预期输出：

```text
=== 迭代器链 ===
evens x10 = [80, 40, 60], sum = 31, max = 9
=== 捕获外部变量 ===
>5 = [8, 9, 6]
=== sort_by ===
by len = ["hi", "Rust", "hi-rust"]
=== Option / Result 与闭包 ===
doubled = Some(84), chained = Some(30), fallback = 100
parse failed: invalid digit found in string
msg = 0
=== 自定义高阶函数 ===
pick_if = [8, 4, 6]
  action
  action
=== 闭包工厂 ===
add5(7) = 12, add10(7) = 17
=== inspect 副作用 ===
[8] [9] [6] 
collected = [3, 8, 1, 9, 4, 6]
```

## 小结

| 场景 | 典型 API |
|------|----------|
| 转换 / 筛选 | `.map`、`.filter`、`.collect` |
| 聚合 | `.sum`、`.reduce`、`.fold` |
| 排序 | `.sort_by`、`.sort_by_key` |
| 可选值 | `.map`、`.and_then`、`.unwrap_or_else` |
| 自定义逻辑 | `fn f(..., impl Fn(...) -> ...)` |
| 生成闭包 | `move` + 返回 `impl Fn` |

## 延伸阅读

- [闭包](/syntax/closures) — 语法、`move`、`Fn` trait
- [迭代器](/syntax/iterators) — 适配器与消费器
- [错误处理](/syntax/error_handling) — `Result` 与闭包组合
- [多线程](/syntax/threads) — `move` 闭包与线程
- [The Rust Book — Closure Usage](https://doc.rust-lang.org/book/ch13-01-closures.html)
