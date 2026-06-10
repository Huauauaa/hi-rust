# 字符串

Rust 里常见的字符串类型有两种：`&str`（字符串切片，常来自字面量）和 `String`（堆上分配、可增长的字符串）。

## 示例代码

源码在仓库根目录的 [`strings.rs`](https://github.com/Huauauaa/hi-rust/blob/main/strings.rs)：

<<< ../../strings.rs{rust}

## 创建与拼接

- `&str` — 不可变引用，指向 UTF-8 字节序列；字面量 `"hello"` 的类型就是 `&str`。
- `String::from` / `push` / `push_str` — 创建可拥有的字符串，并在末尾追加内容。
- `format!` — 按模板拼接，得到新的 `String`。
- `len()` — 字节长度；含中文或 emoji 时与「字符个数」可能不同，用 `chars().count()` 统计 Unicode 标量值。

## 常用方法

### 查询

| 方法 | 说明 |
|------|------|
| `contains(pat)` | 是否包含子串或字符 |
| `starts_with(pat)` | 是否以指定前缀开头 |
| `ends_with(pat)` | 是否以指定后缀结尾 |
| `is_empty()` | 是否为空（长度为 0） |
| `find(pat)` | 返回第一次匹配的 `Option<usize>` 字节索引 |

### 转换

| 方法 | 说明 |
|------|------|
| `to_uppercase()` | 转大写，返回新 `String` |
| `to_lowercase()` | 转小写，返回新 `String` |
| `replace(from, to)` | 替换所有匹配项，返回新 `String` |
| `trim()` / `trim_start()` / `trim_end()` | 去掉首尾空白（在 `&str` 上调用） |

### 分割

| 方法 | 说明 |
|------|------|
| `split(pat)` | 按分隔符迭代出 `&str` 片段 |
| `lines()` | 按行分割 |
| `chars()` | 按 Unicode 字符迭代 |

### 修改（`mut String`）

| 方法 | 说明 |
|------|------|
| `insert(idx, ch)` | 在字节索引处插入字符（须在字符边界上） |
| `pop()` | 移除并返回最后一个字符 |
| `remove(idx)` | 按字节索引移除一个字符 |
| `clear()` | 清空内容 |
| `truncate(len)` | 截断到指定字节长度 |

`String` 也可用 `&s` 或 `s.as_str()` 得到 `&str`，传给只接受字符串切片的 API。

## 运行

在项目根目录执行：

```sh
rustc strings.rs
./strings
```

Windows 下运行 `strings.exe`。

预期输出：

```text
=== 创建与拼接 ===
literal: hello
owned: Rust! 🦀
greeting: hello, Rust! 🦀
字节长度: 17
字符数: 14
=== 查询 ===
contains Rust: true
starts_with hello: true
ends_with 🦀: true
is_empty: false
find !: Some(11)
=== 转换 ===
to_uppercase: HELLO, RUST! 🦀
to_lowercase: hello, rust! 🦀
trim: [hi]
replace: hello, rust! 🦀
=== 分割 ===
part: a
part: b
part: c
=== 修改 ===
insert: hello!
pop: Some('!')
after push_str: hello world
after clear, is_empty: true
```

## 小结

| 类型 | 所有权 | 典型用途 |
|------|--------|----------|
| `&str` | 借用 | 字面量、函数参数、只读片段 |
| `String` | 拥有 | 需要拼接、修改或返回给调用方 |
