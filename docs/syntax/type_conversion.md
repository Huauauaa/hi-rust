# 类型转换

Rust **不做隐式类型转换**（除部分生命周期协变外）。数值互转、字符串解析、trait 驱动的 `From` / `Into` 等都要**显式**写出。转换失败时，优先用 `Result`（如 `parse`、`TryFrom`）而非 silent 截断。

标量与类型概览见 [类型系统](/syntax/type_system)；错误处理见 [错误处理](/syntax/error_handling)。

## 示例代码

源码在仓库根目录的 [`type_conversion.rs`](https://github.com/Huauauaa/hi-rust/blob/main/type_conversion.rs)：

<<< ../../type_conversion.rs{rust}

## `as`：primitive 强制转换

`as` 用于数值、指针等**原始类型**之间的转换，可能**截断**或丢失精度：

```rust
let small = 300_i32 as u8;   // 44，高位被截断
let int_pi = 3.9_f64 as i32; // 3，向零取整
let avg = (a as f64) / (b as f64);
```

| 场景 | 说明 |
|------|------|
| 整数 ↔ 整数 | 可能截断，大类型转小类型需格外小心 |
| 浮点 → 整数 | 截断小数部分 |
| 整数 → 浮点 | 大整数转 `f64` 可能丢失精度 |

`as` 不会返回 `Result`；溢出时静默截断。需要检测失败时用 `TryFrom`。

## `From` 与 `Into`

标准库为常见转换实现 `From<T>`，调用 `From::from(x)` 或 `.into()`：

```rust
let s = String::from("hi-rust");
let s: String = "hello".into();
let c = char::from(65_u8);
```

- `Into<T>` 在实现 `From` 时自动可用。
- 比 `as` 更安全、语义更清晰，优先用于有 `From` 实现的类型。

## `TryFrom` 与 `TryInto`

可能失败的转换返回 `Result`：

```rust
u8::try_from(256_i32);  // Err(...)
let n: i32 = 10_u8.try_into().unwrap();
```

适合「范围检查」类转换；与 [错误处理](/syntax/error_handling) 中的 `?` 配合使用。

## `parse`：字符串解析

将字符串转为数值等类型，返回 `Result`：

```rust
let n: i32 = "42".parse().unwrap();
let f: f64 = "3.14".parse().unwrap();
let n = "42".parse::<i32>()?;  // turbofish 指定类型
```

目标类型须实现 `FromStr` trait（整数、浮点、`bool` 等均支持）。

## `to_string` 与 `Display`

实现了 `Display` 的类型可 `.to_string()` 得到 `String`：

```rust
let text = 100_i32.to_string();
let flag = true.to_string();
```

格式化复杂输出用 `format!`；详见 [字符串](/syntax/strings)。

## `String` 与 `&str`

| 方向 | 写法 |
|------|------|
| `&str` → `String` | `String::from(s)`、`s.to_string()`、`s.into()` |
| `String` → `&str` | `&s`、`s.as_str()` |

`&str` 是借用，`String` 拥有堆数据；转换时注意 [所有权](/syntax/ownership)。

## 集合与切片

```rust
let vec: Vec<i32> = [1, 2, 3].to_vec();
```

数组、切片与 `Vec` 等之间的转换由各自 `From` / 迭代器方法提供，无统一 `as` 规则。

## 运行

在项目根目录执行：

```sh
rustc type_conversion.rs
./type_conversion
```

Windows 下运行 `type_conversion.exe`。

预期输出：

```text
=== as 数值转换 ===
300 as u8 = 44, 3.9 as i32 = 3
avg = 3.33
=== From / Into ===
hi-rust, hello, char = A
=== TryFrom / TryInto ===
err = out of range integral type conversion attempted
u8 -> i32 = 10
=== parse ===
n = 42, f = 3.14, bad = Err(ParseIntError { kind: InvalidDigit })
=== to_string ===
text = 100, flag = true
=== String 与 &str ===
slice = Rust, restored = Rust
=== 切片转 Vec ===
vec = [1, 2, 3]
```

## 小结

| 方式 | 失败时 | 典型用途 |
|------|--------|----------|
| `as` | 静默截断 | primitive 数值互转 |
| `From` / `Into` | 编译期保证 | `String::from`、`into()` |
| `TryFrom` / `TryInto` | `Result` | 有范围限制的转换 |
| `.parse()` | `Result` | 字符串 → 数值 |
| `.to_string()` | — | 值 → `String` |

## 延伸阅读

- [类型系统](/syntax/type_system) — 标量类型与类型一致
- [字符串](/syntax/strings) — `&str` 与 `String`
- [错误处理](/syntax/error_handling) — `Result` 与 `?`
- [The Rust Book — Type Conversions](https://doc.rust-lang.org/reference/type-casts.html)
