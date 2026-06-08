# 循环

Rust 提供 `loop`、`while`、`for` 三种循环。`break` 提前退出，`continue` 跳过本次迭代。`loop` 还可以通过 `break` 带返回值，作为表达式使用。

## 示例代码

源码在仓库根目录的 [`loops.rs`](https://github.com/Huauauaa/hi-rust/blob/main/loops.rs)：

<<< ../../loops.rs{rust}

## loop

`loop` 无限循环，直到遇到 `break`：

```rust
loop {
    count += 1;
    if count >= 3 {
        break;
    }
}
```

`break` 可以带一个值，整个 `loop` 块因此成为表达式：

```rust
let doubled = loop {
    n += 1;
    if n == 5 {
        break n * 2;
    }
};
```

## while

条件为真时重复执行。条件必须是 `bool`：

```rust
while i < 3 {
    print!("{i} ");
    i += 1;
}
```

## for

`for` 遍历迭代器，最常用的是**范围**和**集合**。

- `0..3`：左闭右开，即 `0, 1, 2`。
- `1..=3`：闭区间，即 `1, 2, 3`。
- 直接遍历数组、切片等实现了 `IntoIterator` 的类型。

```rust
for j in 0..3 { ... }
for fruit in fruits { ... }
```

需要索引时，用 `.iter().enumerate()`：

```rust
for (idx, fruit) in fruits.iter().enumerate() {
    println!("{idx}: {fruit}");
}
```

## break 与 continue

- `break`：立刻退出当前循环（`loop` / `while` / `for`）。
- `continue`：跳过本次剩余代码，进入下一次迭代。

```rust
for num in 1..=6 {
    if num % 2 == 0 {
        continue;
    }
    if num > 5 {
        break;
    }
    print!("{num} ");
}
```

## 运行

在项目根目录执行：

```sh
rustc loops.rs
./loops
```

Windows 下运行 `loops.exe`。

预期输出：

```text
=== loop + break ===
count = 3
=== loop 返回值 ===
doubled = 10
=== while ===
0 1 2 
=== for 范围 ===
0 1 2 
1 2 3 
=== for 遍历数组 ===
apple
banana
cherry
=== for 带索引 ===
0: apple
1: banana
2: cherry
=== break / continue ===
1 3 5 
```

## 小结

| 写法 | 说明 |
|------|------|
| `loop { ... }` | 无限循环，用 `break` 退出 |
| `break` / `break val` | 退出循环；`loop` 中可返回值 |
| `while cond { ... }` | 条件为真时重复 |
| `for x in iter { ... }` | 遍历迭代器 |
| `a..b` | 左闭右开范围 |
| `a..=b` | 闭区间范围 |
| `continue` | 跳过本次迭代 |
