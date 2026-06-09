# 集合

标准库提供了多种**集合**类型，用于在运行时存储可变数量的数据。最常用的是 `Vec`（动态数组）、`HashMap`（键值映射）和 `HashSet`（不重复元素集）。

## 示例代码

源码在仓库根目录的 [`collections.rs`](https://github.com/Huauauaa/hi-rust/blob/main/collections.rs)：

<<< ../../collections.rs{rust}

## Vec：动态数组

`Vec<T>` 在堆上分配，长度可在运行时增长或缩短：

```rust
let mut nums = vec![1, 2, 3];
nums.push(4);
nums.pop();
```

| 方法 | 说明 |
|------|------|
| `vec![a, b, c]` | 宏创建并初始化 |
| `Vec::new()` | 创建空向量 |
| `push(val)` | 末尾追加 |
| `pop()` | 移除并返回末尾元素（`Option`） |
| `len()` | 元素个数 |
| `get(i)` | 安全访问，返回 `Option` |

与固定长度[数组](/guide/arrays)相比，`Vec` 更灵活，传给函数时常用切片 `&[T]` 作为参数。

### 遍历

```rust
for n in &nums { ... }           // 不可变借用每个元素
for n in &mut nums { ... }        // 可变借用
for (i, n) in nums.iter().enumerate() { ... }
```

## HashMap：键值映射

`HashMap<K, V>` 存储键到值的映射，键类型须实现 `Eq` 和 `Hash`（如 `String`、`&str`、`i32`）：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 90);
scores.get("Alice");  // Option<&V>
```

| 方法 | 说明 |
|------|------|
| `insert(k, v)` | 插入或覆盖 |
| `get(&k)` | 查询，返回 `Option<&V>` |
| `entry(k).or_insert(v)` | 键不存在时插入默认值 |
| `contains_key(&k)` | 是否包含键 |

同一键重复 `insert` 会覆盖旧值。

## HashSet：集合

`HashSet<T>` 存储**不重复**的元素，适合去重或快速判断是否存在：

```rust
use std::collections::HashSet;

let mut tags = HashSet::new();
tags.insert("rust");
tags.contains("rust");  // bool
```

插入已存在的元素不会增加长度。

## 对比

| 类型 | 特点 | 典型用途 |
|------|------|----------|
| `[T; N]` | 固定长度，栈上 | 已知长度的小数组 |
| `Vec<T>` | 动态长度，堆上 | 列表、序列 |
| `HashMap<K, V>` | 键值对 | 字典、计数、索引 |
| `HashSet<T>` | 唯一元素 | 去重、成员检测 |

## 运行

在项目根目录执行：

```sh
rustc collections.rs
./collections
```

Windows 下运行 `collections.exe`。

预期输出：

```text
=== Vec 创建与增删 ===
nums = [1, 2, 3, 4, 5]
pop = Some(5)
nums = [1, 2, 3, 4]
names = ["Rust", "hi-rust"]
=== Vec 遍历 ===
nums[0] = 1
nums[1] = 2
nums[2] = 3
nums[3] = 4
sum = 10
=== HashMap ===
Alice = Some(90)
Alice: 95
Bob: 85
Carol: 88
=== HashSet ===
len = 2
contains docs: true
docs rust 
```

HashSet 遍历顺序不固定，输出中 `docs rust` 与 `rust docs` 均正常。

## 小结

| 写法 | 说明 |
|------|------|
| `vec![1, 2, 3]` | 创建 `Vec` |
| `Vec::new()` | 空向量 |
| `HashMap::new()` | 空映射 |
| `HashSet::new()` | 空集合 |
| `use std::collections::...` | 引入集合类型 |

## 延伸阅读

- [`数组`](/guide/arrays) — 固定长度 `[T; N]`
- [`切片`](/guide/slices) — `&[T]` 与 `Vec` 配合
- [`所有权和移动`](/guide/ownership) — `Vec`、`String` 在堆上分配
- [The Rust Book — Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
