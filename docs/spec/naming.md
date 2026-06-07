# 命名规范

Rust 社区约定用 **snake_case**、**SCREAMING_SNAKE_CASE**、**PascalCase** 区分不同类别的标识符。`rustfmt` 与 `clippy` 会按这些习惯格式化或提示。

## 示例代码

源码在仓库根目录的 [`naming.rs`](https://github.com/Huauauaa/hi-rust/blob/main/naming.rs)：

<<< ../../naming.rs{rust}

## 基本规则

| 类别 | 风格 | 示例 |
|------|------|------|
| 变量、函数、模块、crate | snake_case | `user_name`、`calculate_total_price` |
| 常量、静态变量 | SCREAMING_SNAKE_CASE | `MAX_CONNECTIONS` |
| 结构体、枚举、trait、类型别名 | PascalCase | `UserProfile`、`HttpStatus`、`Greeter` |
| 枚举变体 | PascalCase | `Ok`、`NotFound` |
| 生命周期 | 撇号 + 小写 | `'a`、`'ctx` |
| 泛型参数 | 单个大写字母或 PascalCase | `T`、`E`、`Item` |

## 变量与函数

- 局部变量、函数参数、函数名一律 **snake_case**。
- 可变绑定同样用 snake_case，通过 `mut` 区分可变性，而不是改名字。
- 函数名通常是动词或动宾短语：`parse_config`、`read_file`。

```rust
let user_name = "alice";
let mut request_count = 0;

fn calculate_total_price(unit_price: u32, quantity: u32) -> u32 { ... }
```

## 类型与 trait

- 结构体、枚举、union、trait、type 别名用 **PascalCase**。
- 枚举变体也用 PascalCase，与类型名风格一致。
- trait 名往往是形容词或能力描述：`Greeter`、`Clone`、`Iterator`。

```rust
struct UserProfile { display_name: String }

enum HttpStatus { Ok, NotFound }

trait Greeter { fn greet(&self, user_name: &str); }
```

## 常量与静态项

编译期常量 `const` 和静态变量 `static` 用 **SCREAMING_SNAKE_CASE**，与普通变量区分：

```rust
const MAX_CONNECTIONS: u32 = 100;
static APP_NAME: &str = "hi-rust";
```

## 模块与 crate

- 模块文件、目录、crate 名：**snake_case**（如 `user_profile.rs`、`hi_rust`）。
- crate 名中的连字符在代码里写成下划线：`use my_crate::foo` 对应包名 `my-crate`。

## 其他约定

| 场景 | 写法 |
|------|------|
| 未使用的变量 / 参数 | 前缀 `_`，如 `_unused`、`_ctx` |
| 只绑定部分字段 | `_` 忽略：`let (x, _) = pair;` |
| 与关键字冲突 | 原始标识符 `r#type` |
| 转换方法 | `as_` / `to_` / `into_` 前缀：`as_str`、`to_string`、`into_inner` |
| getter | 通常不用 `get_` 前缀，直接字段名或同名方法 |

## 运行

在项目根目录执行：

```sh
rustc naming.rs
./naming
```

Windows 下运行 `naming.exe`。

预期输出：

```text
MAX_CONNECTIONS: 100
user: alice, requests: 1
Hello, alice! I'm Alice
total: 30
status: OK
404 variant: 404
```

## 小结

| 风格 | 用于 |
|------|------|
| snake_case | 变量、函数、模块、字段 |
| SCREAMING_SNAKE_CASE | `const`、`static` |
| PascalCase | 类型、trait、枚举变体 |
| `'a` | 生命周期参数 |

官方参考：[Rust API Guidelines — Naming](https://rust-lang.github.io/api-guidelines/naming.html)
