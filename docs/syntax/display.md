# Display

**`Display`** 是标准库中的 trait，用于**面向用户**的可读格式化输出——`println!` 里的 `{}`、`format!`、`to_string()` 都依赖它。与 [Debug](/syntax/debug) 的 `{:?}` 不同，`Display` 需要按业务语义**手写**实现，不能 `derive`。

## 示例代码

源码在仓库根目录的 [`display.rs`](https://github.com/Huauauaa/hi-rust/blob/main/display.rs)：

<<< ../../display.rs{rust}

## `{}` 与用户可读输出

`println!`、`format!` 默认用 `{}`，即 **Display** 格式：

```rust
let s = String::from("Rust");
println!("{s}");      // Rust
println!("{}", 42);   // 42
```

标准库中 `String`、`&str`、整数、浮点等已实现 `Display`。自定义 struct / enum 默认**不能**直接用 `{}` 打印：

```rust
// println!("{user}");  // 编译错误：User 未实现 Display
```

## 手写 `impl Display`

实现 `fmt` 方法，用 `write!` 写入目标 formatter：

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 3, y: 4 };
println!("{p}");  // (3, 4)
```

- `f` 是输出目标，类似一个「写入器」。
- `write!` 返回 `fmt::Result`，用 `?` 向上传递格式化错误（在 `fmt` 里通常直接 `write!` 即可）。
- 输出格式完全由你决定，应表达**业务含义**，而非调试结构。

enum 同样可以 implement，按变体分别格式化：

```rust
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "active"),
            Status::Inactive => write!(f, "inactive"),
            Status::Banned(reason) => write!(f, "banned: {reason}"),
        }
    }
}
```

## `format!` 与 `to_string`

实现 `Display` 后，自动获得 **`ToString`**（通过 blanket impl），可：

```rust
let text = format!("hello, {user}");
let owned = user.to_string();
```

`format!` 与 `println!` 使用同一套格式化规则；需要 `String` 时用 `to_string()` 或 `format!("{value}")`。

## Display 与 Debug

| | `Display` (`{}`) | `Debug` (`:?`) |
|--|------------------|----------------|
| 用途 | 给用户看的输出 | 给开发者调试 |
| 实现 | 手写 `impl Display` | 常 `#[derive(Debug)]` |
| 输出风格 | 自定义、简洁可读 | 结构清晰、偏内部细节 |
| 标准库覆盖 | 常用展示类型 | 几乎所有类型 |

实践中常同时保留两者：对外展示用 `Display`，日志与测试用 `Debug`：

```rust
#[derive(Debug)]
struct User { ... }

impl fmt::Display for User { ... }

println!("{user}");    // Alice (30 yrs)
println!("{user:?}");   // User { name: "Alice", age: 30 }
```

错误类型也常见这种组合——`Display` 给用户看错误信息，`Debug` 给开发者看完整细节，见 [错误处理](/syntax/error_handling)。

## 运行

在项目根目录执行：

```sh
rustc display.rs
./display
```

Windows 下运行 `display.exe`。

预期输出：

```text
=== Display 与 {} ===
point = (3, 4)
(3, 4)
user = Alice (30 yrs)
=== format! 与 to_string ===
hello, Alice (30 yrs)
Alice (30 yrs)
=== 标准库类型 ===
Rust, hi-rust, 42
=== Display 与 Debug 对比 ===
Display: Alice (30 yrs)
Debug:   User { name: "Alice", age: 30 }
```

## 小结

| 写法 | 说明 |
|------|------|
| `{}` / `{value}` | Display 格式输出 |
| `impl fmt::Display for T` | 手写用户可读格式化 |
| `write!(f, "...")` | 在 `fmt` 中写入内容 |
| `format!` / `to_string()` | 实现 Display 后可用 |
| `Display` vs `Debug` | 用户展示 vs 开发调试 |

## 延伸阅读

- [`Debug`](/syntax/debug) — `{:?}` 与 `#[derive(Debug)]`
- [`字符串`](/syntax/strings) — `String` 与 `&str`
- [`结构体`](/syntax/structs) — 自定义类型
- [`错误处理`](/syntax/error_handling) — 错误信息的展示
- [The Rust Book — Display](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
