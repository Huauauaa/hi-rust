# 变量绑定

Rust 里「变量」是**绑定**（binding）：名字与值关联，而非随意指向的盒子。`let` 不仅声明名字，还可用**模式**解构元组、结构体和枚举；遮蔽（shadowing）则创建新绑定覆盖同名变量。

基础 `let` / `mut` / `const` 见 [定义变量](/syntax/variables)。

## 示例代码

源码在仓库根目录的 [`bindings.rs`](https://github.com/Huauauaa/hi-rust/blob/main/bindings.rs)：

<<< ../../bindings.rs{rust}

## 遮蔽（shadowing）

再次 `let` 同名变量会**遮蔽**旧绑定，旧值仍有效直到其 scope 结束：

```rust
let x = 5;
let x = x + 1;   // 新绑定，x 为 6
```

遮蔽与 `mut` 重新赋值的区别：

| | 遮蔽 | `let mut` + 赋值 |
|--|------|------------------|
| 新绑定 | 是 | 否，修改同一绑定 |
| 可改类型 | 是 | 否，类型固定 |
| 示例 | `let s = " "; let s = s.len();` | `let mut n = 0; n += 1;` |

## 解构元组与 struct

`let` 左侧可以是模式，一次取出多个字段：

```rust
let (a, b) = (10, 20);

let Point { x, y } = point;
let Point { x: px, .. } = point;  // .. 忽略其余字段
```

字段名与变量名相同时可简写：`Point { x, y }` 等价于 `Point { x: x, y: y }`。

## 解构 enum

`match` 与 `if let` 在分支里绑定变体内部数据：

```rust
if let Message::Text(text) = msg {
    println!("{text}");
}

match msg {
    Message::Text(t) => { ... }
    Message::Quit => { ... }
}
```

详见 [枚举](/syntax/enums) 与 [条件判断](/syntax/conditionals)。

## `ref` 与 `ref mut`

模式里用 `ref` 绑定**引用**，避免移动原值：

```rust
let value = 42;
let ref r = value;      // r 是 &i32，value 仍可用

let mut point = Point { x: 1, y: 2 };
let Point { ref mut x, .. } = point;
*x += 10;               // 通过可变引用修改 point.x
```

等价于对字段使用 `&` / `&mut`，但在模式里更直观。

## 忽略与占位

| 写法 | 说明 |
|------|------|
| `_` | 忽略单个值，不绑定变量 |
| `..` | 忽略 struct 剩余字段 |
| `_name` | 绑定但故意不使用，抑制未使用警告 |

```rust
let (_a, used) = (1, 2);
let Point { x: _, y } = point;
```

## 运行

在项目根目录执行：

```sh
rustc bindings.rs
./bindings
```

Windows 下运行 `bindings.exe`。

预期输出：

```text
=== 遮蔽与类型变化 ===
spaces = 3
=== 解构元组 ===
a = 10, b = 20
=== 解构 struct ===
x = 3, y = 4
px = 7
=== 解构 enum ===
text = hi-rust
action = quit
=== ref 绑定 ===
value = 42, r = 42
=== 忽略值 _ ===
used = 200
=== ref mut 解构 ===
point.x = 11
```

## 小结

| 写法 | 说明 |
|------|------|
| `let x = ...` | 不可变绑定 |
| `let x = ...; let x = ...` | 遮蔽，可换类型 |
| `let (a, b) = tuple` | 解构元组 |
| `let Struct { f, .. } = s` | 解构 struct |
| `if let` / `match` | 解构 enum |
| `ref` / `ref mut` | 绑定引用而非移动 |
| `_` / `..` | 忽略部分值 |

## 延伸阅读

- [定义变量](/syntax/variables) — `let`、`mut`、`const` 基础
- [`元组`](/syntax/tuples) — 元组类型与解构
- [`结构体`](/syntax/structs) — 字段与 struct 模式
- [`枚举`](/syntax/enums) — enum 变体绑定
- [ref 解构有什么用？](/qa/ref_destructuring) — 何时用 `ref` / `ref mut`
- [The Rust Book — Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
