# IO

Rust 通过标准库 `std::io` 和 `std::fs` 处理**输入输出**：终端上的标准流、以及读写文件。常见 API 返回 `Result`，需要处理可能的错误。

## 示例代码

源码在仓库根目录的 [`io.rs`](https://github.com/Huauauaa/hi-rust/blob/main/io.rs)：

<<< ../../io.rs{rust}

## 标准输出

| 宏 / API | 说明 |
|----------|------|
| `println!(...)` | 打印一行并换行 |
| `print!(...)` | 打印不换行 |
| `eprintln!(...)` | 写到标准错误（stderr） |
| `format!(...)` | 格式化字符串，不打印 |

`print!` 不会自动刷新缓冲区；若紧接着要从 stdin 读入，可调用 `io::stdout().flush()` 确保内容立刻显示。

## 标准输入

从终端读一行：

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input)?;
let trimmed = input.trim();
```

- `read_line` 会把换行符 `\n` 一并读入，通常用 `trim()` 去掉首尾空白。
- 需要数字时用 `trimmed.parse::<i32>()`，得到 `Result`。

示例里用 `Cursor` 模拟输入流，便于直接 `rustc io.rs && ./io` 运行；真实程序把 `Cursor` 换成 `io::stdin()` 即可。

## 文件读写

| API | 说明 |
|-----|------|
| `fs::write(path, contents)` | 写入整个文件（覆盖） |
| `fs::read_to_string(path)` | 读入整个文件为 `String` |
| `fs::read(path)` | 读入为 `Vec<u8>` |
| `fs::remove_file(path)` | 删除文件 |

路径可以是相对路径（相对当前工作目录）或绝对路径。

## Result 与 `?`

IO 操作可能失败（磁盘满、文件不存在、权限不足等），因此返回 `Result<T, E>`：

```rust
fn main() -> io::Result<()> {
    let text = fs::read_to_string("config.txt")?;
    println!("{text}");
    Ok(())
}
```

- `?` 在出错时提前返回，把错误传给调用方。
- `main` 返回 `io::Result<()>` 时，程序失败会以非零退出码结束。

详见 [错误处理](/syntax/error_handling)。示例里也可用 `.expect("msg")` 快速处理错误；正式代码更推荐 `?` 或 `match`。

## 运行

在项目根目录执行：

```sh
rustc io.rs
./io
```

Windows 下运行 `io.exe`。

预期输出：

```text
=== 标准输出 ===
Hello, Rust!
(stderr) 日志：程序已启动
=== 标准输入（模拟） ===
name = Alice
age = 30
=== 文件读写 ===
file contents:
hi-rust
IO demo
```

## 小结

| 写法 | 说明 |
|------|------|
| `println!` / `print!` | 标准输出 |
| `io::stdin().read_line(&mut s)` | 读一行 |
| `fs::write` / `fs::read_to_string` | 写文件 / 读文件 |
| `main() -> io::Result<()>` + `?` | 向上传播 IO 错误 |

## 延伸阅读

- [`Hello, World`](/syntax/hello_world) — 第一个 `println!`
- [`字符串`](/syntax/strings) — `String` 与 `&str`
- [`函数`](/syntax/functions) — 返回 `Result` 的函数
- [文件操作](/syntax/file_ops) — 目录、元数据、复制与遍历
- [The Rust Book — I/O](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
