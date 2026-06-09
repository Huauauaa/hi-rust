# 定义变量

Rust 用 `let` 绑定变量。默认不可变；需要修改时用 `mut`。常量用 `const` 声明。

## 示例代码

源码在仓库根目录的 [`variables.rs`](https://github.com/Huauauaa/hi-rust/blob/main/variables.rs)：

<<< ../../variables.rs{rust}

- `let name = "Rust"` — 不可变绑定，类型由编译器推断。
- `let mut count = 0` — 可变绑定，可以 `count += 1`。
- `const MAX_POINTS: u32 = 100_000` — 编译期常量，必须标注类型。
- `let x = 5` 后再 `let x = x + 1` — 遮蔽（shadowing），用新绑定覆盖同名变量。

## 运行

在项目根目录执行：

```sh
rustc variables.rs
./variables
```

Windows 下运行 `variables.exe`。

预期输出：

```text
name: Rust
count: 1
MAX_POINTS: 100000
x: 6
```

## 小结

| 写法 | 可否重新赋值 | 说明 |
|------|-------------|------|
| `let` | 否 | 默认不可变 |
| `let mut` | 是 | 显式可变 |
| `const` | 否 | 编译期常量，需写类型 |
