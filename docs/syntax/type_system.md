# 类型系统

Rust 是**静态类型**语言：每个表达式的类型在编译期确定。编译器大多能**推断**类型，必要时也可显式标注。类型不一致会在编译阶段报错，而不是运行时悄悄出错。

## 示例代码

源码在仓库根目录的 [`type_system.rs`](https://github.com/Huauauaa/hi-rust/blob/main/type_system.rs)：

<<< ../../type_system.rs{rust}

## 标量类型

| 类型 | 说明 | 示例 |
|------|------|------|
| 有符号整数 | `i8` … `i128`，默认整数字面量 `i32` | `-42_i32` |
| 无符号整数 | `u8` … `u128` | `42_u32` |
| 浮点 | `f32`、`f64`，默认 `f64` | `3.14` |
| 布尔 | `bool` | `true` / `false` |
| 字符 | `char`，4 字节 Unicode | `'中'` |

整数与浮点、有符号与无符号是**不同类型**，不能隐式混算（如 `i32 + u32` 需显式转换）。

## 类型推断与标注

```rust
let inferred = 100;        // 推断为 i32
let explicit: u16 = 100;   // 显式标注
```

函数参数与返回值通常需要写明类型；函数体内局部变量可省略，由编译器推断：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

推断失败时编译器会提示补充标注。

## 单元类型 `()`

无有用返回值时使用单元类型 `()`，例如：

```rust
let unit: () = ();
```

不写返回类型的函数等价于返回 `()`。

## 复合类型概览

| 类别 | 示例 | 详见 |
|------|------|------|
| 元组 | `(i32, &str)` | [元组](/syntax/tuples) |
| 数组 | `[i32; 3]` | [数组](/syntax/arrays) |
| 切片 | `&[i32]` | [切片](/syntax/slices) |
| 字符串 | `String`、`&str` | [字符串](/syntax/strings) |
| 结构体 | `struct Point { x: i32 }` | [结构体](/syntax/structs) |
| 枚举 | `enum Option<T>` | [枚举](/syntax/enums) |

同一「形状」的不同类型参数也是不同类型，如 `Vec<i32>` 与 `Vec<String>`。

## 类型别名

用 `type` 为已有类型起别名，提高可读性，**不创建新类型**：

```rust
type UserId = u64;

let id: UserId = 1001;
```

别名与原名可互换；泛型别名写法见 [泛型](/syntax/generics)。

## 类型转换

Rust **没有**隐式 widening。常用方式：

| 方式 | 说明 |
|------|------|
| `as` | 数值等 primitive 之间的显式转换（可能截断） |
| `.parse()` | 字符串解析，返回 `Result` |
| `.into()` / `From` | trait 驱动的转换 |

```rust
let byte = 65_i32 as u8;
let n: i32 = "42".parse()?;
let s: String = "hi".into();
```

详见 [类型转换](/syntax/type_conversion) 章节。

## 类型一致性

`if`、`match` 作为表达式时，各分支须为**同一类型**：

```rust
let v = if flag { 1_i32 } else { 2 };
```

函数调用也会检查实参类型是否与形参匹配；不匹配时编译器给出错误位置与期望类型。

## 泛型与 trait

参数化类型（`Vec<T>`、`Option<T>`）与 trait 约束是类型系统的延伸，见 [泛型](/syntax/generics) 章节。

## 运行

在项目根目录执行：

```sh
rustc type_system.rs
./type_system
```

Windows 下运行 `type_system.exe`。

预期输出：

```text
=== 标量类型 ===
-42, 42, 3.14, true, 中
=== 类型推断 ===
inferred = 100, explicit = 100
describe = i32: 100
=== 单元类型 ===
unit = ()
=== 类型别名 ===
UserId = 1001
=== as 转换 ===
byte = 65, char = A
=== parse / into ===
parsed = 42, name = hi-rust
=== 复合类型 ===
pair = (1, "tuple"), arr = [1, 2, 3]
=== 表达式须同一类型 ===
v = 1
=== 不同类型不可混用 ===
u32 sum = 3
```

## 小结

| 概念 | 说明 |
|------|------|
| 静态类型 | 编译期检查，运行时无类型信息开销 |
| 类型推断 | 省略局部变量类型标注 |
| 标量 / 复合 | 整数、浮点、bool、char；元组、数组、struct 等 |
| `type` 别名 | 可读性，非新类型 |
| 显式转换 | 见 [类型转换](/syntax/type_conversion) |
| 分支同型 | `if` / `match` 表达式各分支类型一致 |

## 延伸阅读

- [定义变量](/syntax/variables) — 绑定与 mut
- [变量绑定](/syntax/bindings) — 遮蔽可改变类型
- [类型转换](/syntax/type_conversion) — `as`、`From`、`parse`
- [泛型](/syntax/generics) — 类型参数与约束
- [The Rust Book — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
