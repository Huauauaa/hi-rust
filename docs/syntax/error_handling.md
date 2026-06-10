# 错误处理

Rust 没有异常机制，用 **`Option<T>`** 表示可能缺失的值，用 **`Result<T, E>`** 表示可能失败的操作。编译器要求显式处理 `None` 和 `Err`，避免忽略错误。

## 示例代码

源码在仓库根目录的 [`error_handling.rs`](https://github.com/Huauauaa/hi-rust/blob/main/error_handling.rs)：

<<< ../../error_handling.rs{rust}

## Option：值可能不存在

`Option<T>` 有两个变体：`Some(T)` 和 `None`（见 [枚举](/syntax/enums)）。

| 方法 | 说明 |
|------|------|
| `unwrap_or(default)` | `None` 时返回默认值 |
| `map(f)` | 有值时变换内部值 |
| `and_then(f)` | 有值时链式返回新 `Option` |
| `is_some()` / `is_none()` | 判断是否有值 |
| `if let Some(x) = opt` | 只处理有值分支 |

`unwrap()` 在 `None` 时 **panic**，仅适合原型或确定不会失败的情况。

## Result：操作可能失败

`Result<T, E>` 有 `Ok(T)` 和 `Err(E)` 两个变体。解析、IO 等 API 常用 `Result`：

```rust
let n: Result<i32, _> = "42".parse();
let text = fs::read_to_string("config.txt");
```

用 `match` 分别处理成功与失败：

```rust
match parse_age("30") {
    Ok(age) => println!("{age}"),
    Err(e) => println!("{e}"),
}
```

## `?` 运算符

在返回 `Result` 的函数里，`?` 在 `Err` 时**提前返回**错误，在 `Ok` 时解包值：

```rust
fn run() -> Result<u32, ParseIntError> {
    let n = "42".parse::<u32>()?;
    Ok(n * 2)
}
```

`?` 要求错误类型能转换为函数的 `Err` 类型（通过 `From` 或相同类型）。`main` 也可返回 `Result`，失败时以非零退出码结束。

## 快捷方法

| 方法 | 说明 |
|------|------|
| `unwrap_or(default)` | 失败时返回默认值 |
| `unwrap_or_else(f)` | 失败时用闭包生成默认值 |
| `expect("msg")` | 失败时 panic 并附带消息 |
| `map_err(f)` | 变换 `Err` 类型或错误信息 |

生产代码优先 `?` 和 `match`；`expect` / `unwrap` 适合示例或逻辑上不可能失败的路径。

## 自定义错误

`Result` 的错误类型 `E` 可以是任意类型，例如 `&str` 或自定义枚举：

```rust
fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}
```

大型项目常用 `thiserror`、`anyhow` 等 crate 组织错误（需 Cargo 项目）。

## IO 与错误

文件读写返回 `io::Result<T>`（即 `Result<T, io::Error>`），用法相同：

```rust
let text = fs::read_to_string("config.txt")?;
```

详见 [IO](/syntax/io) 与 [文件操作](/syntax/file_ops)。

## 运行

在项目根目录执行：

```sh
rustc error_handling.rs
./error_handling
```

Windows 下运行 `error_handling.exe`。

预期输出：

```text
=== Option ===
unwrap_or = 10
map = Some(20)
and_then = Some(10)
if let Some = 10
first_word = hello
none is_none = true
=== Result 与 match ===
age = 30
parse error: invalid digit found in string
=== ? 运算符 ===
? result = 84
=== unwrap_or / expect ===
unwrap_or default = 0
expect ok = 7
=== 自定义错误信息 ===
10 / 2 = 5
error: division by zero
=== IO 错误 ===
file text: hi-rust
=== map_err ===
invalid age: invalid digit found in string
```

## 小结

| 写法 | 说明 |
|------|------|
| `Option<T>` | 有值 / 无值 |
| `Result<T, E>` | 成功 / 失败 |
| `match` / `if let` | 显式分支处理 |
| `?` | 错误时提前返回 |
| `unwrap_or` / `expect` | 快捷处理（慎用 `unwrap`） |
| `map` / `map_err` | 变换 Ok / Err 内容 |

## 延伸阅读

- [`枚举`](/syntax/enums) — `Option` 的定义与 `match`
- [`条件判断`](/syntax/conditionals) — `match`、`if let`
- [IO](/syntax/io) — `io::Result` 与 `?`
- [The Rust Book — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
