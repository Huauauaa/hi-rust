# 单引号与双引号有区别吗？

**有，而且区别很大。** 在 Rust 里，双引号和单引号不是「两种写字符串的方式」，而是两种完全不同的字面量：

| 写法 | 类型 | 含义 |
|------|------|------|
| `"hello"` | `&str` | **字符串**（字符串切片引用） |
| `'h'` | `char` | **单个字符**（Unicode 标量值） |

这和 JavaScript、Python 等语言不同——在那些语言里单引号、双引号往往都能表示字符串。Rust 里**只有双引号**用来写字符串字面量。

## 双引号 `"..."`：字符串

双引号包裹的是**字符串**，类型为 `&str`：

```rust
let s = "hello";
let name: &str = "Rust";
```

- 可以是任意长度，包括空字符串 `""`。
- 内容是 UTF-8 编码的字节序列。
- 详见语法章节：[字符串](/guide/strings)。

## 单引号 `'...'`：字符

单引号包裹的是**单个字符**，类型为 `char`：

```rust
let c = 'a';
let emoji = '🦀';
let zh = '中';
```

- 只能有一个 Unicode 字符，不能写 `'ab'`（编译错误）。
- 不能写空字符 `''`（编译错误）。
- `char` 占 4 字节，能表示任意 Unicode 标量值。

## 不能互换

```rust
let c: char = 'a';
let s: &str = "a";

// let c: char = "a";   // 错误：期望 char，得到 &str
// let s: &str = 'a';   // 错误：期望 &str，得到 char
```

从 `"a"` 取第一个字符不能直接用 `[0]`（字符串按字节索引，且 `'a'` 与 `"a"` 类型不同），应使用：

```rust
let s = "a";
let c = s.chars().next().unwrap(); // char
```

或字面量直接写 `'a'`。

## 常见用法

| 场景 | 写法 |
|------|------|
| 字符串变量、函数参数 | `"hello"` |
| 单个字母、符号 | `'x'`、`'\n'` |
| `String` 追加字符 | `s.push('!')` |
| `String` 追加字符串 | `s.push_str("!")` |
| 字符比较 | `if c == 'A' { ... }` |
| 字符串比较 | `if s == "hi" { ... }` |

转义在两者中类似：`'\n'`、`"\n"` 都是换行；`'\''` 表示单引号字符，`"\""` 表示双引号字符。

## 从其他语言迁移

| 语言 | 习惯 | Rust 对应 |
|------|------|-----------|
| JavaScript | `'hi'` 或 `"hi"` 都是字符串 | 只有 `"hi"` 是字符串 |
| Python | `'a'` 单字符也是 str | `'a'` 是 `char`，`"a"` 是 `&str` |
| C | `'a'` 是 int | `'a'` 是 `char`（Unicode） |

## 小结

- **双引号** → 字符串 `&str`（或配合 `String::from` 得到 `String`）。
- **单引号** → 单个字符 `char`。
- Rust **没有**「用单引号写字符串」的语法；记错引号类型是初学者最常见的报错之一。

## 延伸阅读

- [`字符串`](/guide/strings) — `&str` 与 `String`
- [The Rust Book — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#character-type)
