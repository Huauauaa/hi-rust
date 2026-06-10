# 函数

用 `fn` 定义函数。参数和返回值都要写明类型；函数体最后一条**表达式**（无分号）会作为返回值。不写返回类型时，默认返回单元类型 `()`。

## 示例代码

源码在仓库根目录的 [`functions.rs`](https://github.com/Huauauaa/hi-rust/blob/main/functions.rs)：

<<< ../../functions.rs{rust}

## 定义与调用

```rust
fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- `fn` 后跟函数名，参数列表形如 `name: Type`。
- 有返回值时写 `-> Type`；无返回值可省略，等价于 `-> ()`。
- 调用时按 `函数名(实参1, 实参2)` 传入参数。

## 表达式返回值

函数体末尾的表达式不加 `;`，其值即为返回值：

```rust
fn double(x: i32) -> i32 {
    x * 2
}
```

`if` 作为表达式时，各分支类型须一致，也可直接作为返回值：

```rust
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
```

## return 提前返回

需要在中途退出时，用 `return` 显式返回：

```rust
fn abs_diff(a: i32, b: i32) -> i32 {
    if a >= b {
        return a - b;
    }
    b - a
}
```

`return` 后的表达式同样不能加分号（`return a - b;` 里的分号是结束 `return` 语句，不是给返回值表达式加的）。

## 引用参数

不想取得所有权时，参数可以用引用类型，例如 `&str` 表示字符串切片引用：

```rust
fn print_twice(msg: &str) {
    println!("{msg}");
    println!("{msg}");
}
```

调用时直接传变量或字面量，Rust 会自动借用。

## 无返回值

只产生副作用（如打印）、不返回有用值时，省略 `->` 即可：

```rust
fn log_status(ok: bool) {
    if ok {
        println!("status: OK");
    } else {
        println!("status: FAIL");
    }
}
```

## 运行

在项目根目录执行：

```sh
rustc functions.rs
./functions
```

Windows 下运行 `functions.exe`。

预期输出：

```text
=== 调用函数 ===
Hello, Rust!
3 + 5 = 8
=== 表达式返回值 ===
double(4) = 8
max(7, 3) = 7
=== return 提前返回 ===
abs_diff(10, 3) = 7
abs_diff(3, 10) = 7
=== 引用参数 ===
hi-rust
hi-rust
=== 无返回值 ===
status: OK
status: FAIL
```

## 小结

| 写法 | 说明 |
|------|------|
| `fn name() { ... }` | 无参、无返回值（`()`） |
| `fn name(x: T) -> R { ... }` | 带类型注解的参数与返回类型 |
| 末尾表达式 | 无分号，作为返回值 |
| `return val` | 提前返回 |
| `&str` 等引用 | 借用参数，不取得所有权 |
