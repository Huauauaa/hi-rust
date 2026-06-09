# 数组

数组（array）是**固定长度**、**元素类型相同**的连续内存块。类型写作 `[T; N]`，其中 `T` 是元素类型，`N` 是长度（编译期常量）。

## 示例代码

源码在仓库根目录的 [`arrays.rs`](https://github.com/Huauauaa/hi-rust/blob/main/arrays.rs)：

<<< ../../arrays.rs{rust}

## 创建与访问

```rust
let nums = [10, 20, 30, 40, 50];
let zeros: [i32; 5] = [0; 5];
```

- `[a, b, c]` — 列出各元素；长度由元素个数推断。
- `[值; 长度]` — 用同一值填充固定长度，例如 `[0; 5]` 得到五个 `0`。
- 需要明确类型时写 `[T; N]`，如 `[i32; 5]`。
- 用 `[index]` 按下标访问，下标从 `0` 开始；越界会在运行时 panic。
- `.len()` 返回元素个数。

## 可变数组

数组本身不可变时不能改元素；加 `mut` 后可修改：

```rust
let mut scores = [90, 85, 88];
scores[1] = 92;
```

## 遍历

`for item in arr` 会**按值复制**各元素（对实现了 `Copy` 的类型如 `i32` 没问题）：

```rust
for day in days {
    print!("{day} ");
}
```

需要同时拿到下标时，用 `.iter().enumerate()`：

```rust
for (i, day) in days.iter().enumerate() {
    println!("days[{i}] = {day}");
}
```

## 作为参数

数组传给函数时类型须包含长度，例如 `[i32; 5]` 与 `[i32; 3]` 是不同类型：

```rust
fn sum(nums: [i32; 5]) -> i32 { ... }
```

## 数组与向量

| 类型 | 长度 | 典型场景 |
|------|------|----------|
| `[T; N]` | 编译期固定 | 已知长度、栈上小数组 |
| `Vec<T>` | 运行时可变 | 长度不确定、需要增删 |

需要动态长度时用 `Vec`；本章先掌握固定长度数组即可。

## 运行

在项目根目录执行：

```sh
rustc arrays.rs
./arrays
```

Windows 下运行 `arrays.exe`。

预期输出：

```text
=== 创建与访问 ===
nums[0] = 10
nums[4] = 50
len = 5
zeros = [0, 0, 0, 0, 0]
=== 可变数组 ===
scores = [90, 92, 88]
=== 遍历 ===
Mon Tue Wed 
days[0] = Mon
days[1] = Tue
days[2] = Wed
=== 作为参数 ===
sum = 150
```

## 小结

| 写法 | 说明 |
|------|------|
| `[1, 2, 3]` | 创建数组，元素类型须一致 |
| `[0; 5]` | 五个相同元素 |
| `[T; N]` | 显式类型注解 |
| `arr[i]` | 按下标访问 |
| `arr.len()` | 元素个数 |
| `let mut arr` | 允许修改元素 |
