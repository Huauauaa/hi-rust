# 字符串

Rust 里常见的字符串类型有两种：`&str`（字符串切片，常来自字面量）和 `String`（堆上分配、可增长的字符串）。

## 示例代码

源码在仓库根目录的 [`strings.rs`](https://github.com/Huauauaa/hi-rust/blob/main/strings.rs)：

<<< ../../strings.rs{rust}

- `&str` — 不可变引用，指向 UTF-8 字节序列；字面量 `"hello"` 的类型就是 `&str`。
- `String::from("Rust")` — 创建可拥有的 `String`，内容在堆上。
- `push` / `push_str` — 在末尾追加单个字符或字符串片段。
- `format!` — 按模板拼接，得到新的 `String`。
- `len()` — 字节长度；含中文或 emoji 时与「字符个数」可能不同，用 `chars().count()` 统计 Unicode 标量值。

## 运行

在项目根目录执行：

```sh
rustc strings.rs
./strings
```

Windows 下运行 `strings.exe`。

预期输出：

```text
literal: hello
owned: Rust! 🦀
greeting: hello, Rust! 🦀
字节长度: 17
字符数: 14
```

## 小结

| 类型 | 所有权 | 典型用途 |
|------|--------|----------|
| `&str` | 借用 | 字面量、函数参数、只读片段 |
| `String` | 拥有 | 需要拼接、修改或返回给调用方 |
