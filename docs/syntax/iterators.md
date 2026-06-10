# 迭代器

**迭代器**（iterator）按顺序逐个产生值。Rust 里 `for` 循环、链式调用 `map` / `filter` 等都建立在 `Iterator` trait 上。迭代器是**惰性的**：在 `collect`、`for`、`sum` 等「消费」之前，中间步骤不会真正执行。

## 示例代码

源码在仓库根目录的 [`iterators.rs`](https://github.com/Huauauaa/hi-rust/blob/main/iterators.rs)：

<<< ../../iterators.rs{rust}

## 三种迭代方式

| 方法 | 说明 |
|------|------|
| `iter()` | 不可变借用每个元素，得到 `&T` |
| `iter_mut()` | 可变借用每个元素，得到 `&mut T` |
| `into_iter()` | 取得所有权，消耗集合 |

```rust
for n in &vec { ... }        // 等价于 vec.iter()
for n in &mut vec { ... }    // 等价于 vec.iter_mut()
for n in vec { ... }         // 等价于 vec.into_iter()
```

`Vec`、`HashMap` 等集合都提供这三种方法；数组和切片用 `.iter()`。

## 常用适配器

适配器返回新迭代器，可链式调用：

| 方法 | 说明 |
|------|------|
| `map(closure)` | 对每个元素变换 |
| `filter(closure)` | 保留使闭包为 `true` 的元素 |
| `enumerate()` | 附带索引 `(0, item)` |
| `zip(other)` | 与另一迭代器配对（以较短者为准） |
| `take(n)` | 只取前 `n` 个 |
| `skip(n)` | 跳过前 `n` 个 |
| `chain(other)` | 拼接两个迭代器 |

## 消费迭代器

| 方法 | 说明 |
|------|------|
| `collect()` | 收集为 `Vec` 等集合 |
| `sum()` | 求和（元素须实现 `Sum`） |
| `fold(init, f)` | 从左到右累积 |
| `any(f)` / `all(f)` | 是否存在 / 是否全部满足 |
| `find(f)` | 返回第一个匹配的 `Option` |
| `count()` | 元素个数 |

`collect` 常需标注类型：`let v: Vec<i32> = iter.collect();`，或用 `let v: Vec<_> = ...` 让编译器推断元素类型。

## 闭包参数

对 `nums.iter()`（`Iterator<Item = &i32>`），闭包参数常见写法：

```rust
.filter(|&&n| n % 2 == 0)   // 模式匹配解引用
.map(|&n| n * 10)
```

也可写 `|n| *n % 2 == 0`，视元素类型而定。

## 惰性求值

```rust
let doubled = nums.iter().map(|&n| n * 2); // 尚未计算
let result: Vec<_> = doubled.collect();    // 此时才遍历 nums
```

中间适配器只构建计算链，真正读取数据发生在 `collect`、`for`、`sum` 等消费操作时。

## 运行

在项目根目录执行：

```sh
rustc iterators.rs
./iterators
```

Windows 下运行 `iterators.exe`。

预期输出：

```text
=== iter 与 for ===
1 2 3 4 5 
=== iter_mut ===
doubled = [2, 4, 6]
=== map 与 filter ===
evens x10 = [20, 40]
=== enumerate 与 zip ===
0: 1
1: 2
2: 3
1-a
2-b
3-c
=== fold 与 sum ===
sum = 15
product = 120
=== take / skip / chain ===
skip 1 take 3 = [2, 3, 4]
chain = [1, 2, 3, 4]
=== any / all / find ===
any > 4: true
all > 0: true
find == 3: Some(3)
=== into_iter 消耗所有权 ===
Rust
hi-rust
```

## 小结

| 写法 | 说明 |
|------|------|
| `.iter()` / `.iter_mut()` / `.into_iter()` | 三种遍历方式 |
| `.map` / `.filter` | 变换与筛选 |
| `.collect()` | 消费并收集结果 |
| `.fold` / `.sum` | 聚合计算 |
| 链式调用 | 适配器可连续拼接 |

## 延伸阅读

- [`集合`](/syntax/collections) — `Vec` 与 `for` 遍历
- [`循环`](/syntax/loops) — `for`、`while` 控制流
- [`闭包`](/syntax/closures) — 迭代器常配合闭包使用
- [闭包的使用](/syntax/closure_usage) — 迭代器链与高阶用法
- [The Rust Book — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
