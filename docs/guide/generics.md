# 泛型

**泛型**（generics）让函数、结构体、枚举在保持类型安全的前提下复用于多种类型。编译器会为每种具体类型**单态化**（monomorphization）生成专用代码，运行时无额外开销。

## 示例代码

源码在仓库根目录的 [`generics.rs`](https://github.com/Huauauaa/hi-rust/blob/main/generics.rs)：

<<< ../../generics.rs{rust}

## 泛型函数

在函数名后、参数前声明类型参数 `<T>`：

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // ...
}
```

- `T` 是占位类型，调用时由实参推断，如 `largest(&nums)` 推断 `T` 为 `i32`。
- **trait 约束** `T: PartialOrd` 表示 `T` 必须实现 `PartialOrd`，才能用 `>` 比较。

同一函数可处理 `i32`、`char` 等不同类型，无需重复编写。

## 泛型 struct

结构体字段可以是泛型：

```rust
struct Point<T> {
    x: T,
    y: T,
}

let int_p = Point { x: 5, y: 10 };
let float_p = Point { x: 1.0, y: 4.0 };
```

`Point<i32>` 与 `Point<f64>` 是不同类型。标准库中的 `Vec<T>`、`Option<T>` 都是泛型结构体/枚举。

## impl 泛型

为泛型结构体实现方法时，在 `impl` 后也要声明类型参数：

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

也可以为**具体类型**单独实现（如 `impl Point<f64> { ... }`），提供该类型专属的方法。

## 多个类型参数

函数、结构体可以声明多个泛型参数：

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

let pair = Pair { first: 1, second: "Rust" };
```

## 泛型 enum

枚举同样支持泛型：

```rust
enum Maybe<T> {
    Some(T),
    None,
}
```

标准库的 `Option<T>`、`Result<T, E>` 就是泛型枚举，在[枚举](/guide/enums)一章已有接触。

## 为何使用泛型

| 方式 | 问题 |
|------|------|
| 每种类型写一份函数 | 代码重复 |
| 统一用 `i32` 等固定类型 | 丢失类型安全、不适用所有场景 |
| **泛型** | 一份代码、编译期检查、零运行时开销 |

## 运行

在项目根目录执行：

```sh
rustc generics.rs
./generics
```

Windows 下运行 `generics.exe`。

预期输出：

```text
=== 泛型函数 ===
largest int = 100
largest char = y
=== 泛型 struct ===
int: (5, 10), x = 5
float: (1, 4)
=== 多个类型参数 ===
1, Rust
=== 泛型 enum ===
Some(5), Some("hi"), None
```

## 小结

| 写法 | 说明 |
|------|------|
| `fn f<T>(x: T)` | 泛型函数 |
| `T: Trait` | trait 约束 |
| `struct S<T> { ... }` | 泛型结构体 |
| `enum E<T> { ... }` | 泛型枚举 |
| `impl<T> S<T>` | 为泛型类型实现方法 |

## 延伸阅读

- [`函数`](/guide/functions) — 参数与返回类型
- [`结构体`](/guide/structs) — `impl` 与方法
- [`枚举`](/guide/enums) — `Option<T>`、`Result<T, E>`
- [`集合`](/guide/collections) — `Vec<T>`、`HashMap<K, V>`
- [The Rust Book — Generics](https://doc.rust-lang.org/book/ch10-00-generics.html)
