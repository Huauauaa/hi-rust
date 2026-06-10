# 结构体

**结构体**（struct）把命名字段组合成自定义类型，适合表示具有多个相关属性的数据。可以为结构体实现方法（`impl`），组织数据与行为。

## 示例代码

源码在仓库根目录的 [`structs.rs`](https://github.com/Huauauaa/hi-rust/blob/main/structs.rs)：

<<< ../../structs.rs{rust}

## 定义与创建

用 `struct` 声明字段名和类型：

```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}
```

创建实例时按 `字段名: 值` 填入，字段顺序不必与定义一致：

```rust
let user = User {
    name: String::from("Alice"),
    age: 30,
    active: true,
};
```

字段名与变量名相同时可简写：`{ age }` 等价于 `{ age: age }`。

用 `.` 访问字段：`user.name`、`user.age`。

## 可变 struct

修改字段需要实例本身可变：

```rust
let mut alice = User::new("Alice", 30);
alice.age += 1;
```

整个实例标记 `mut`，而不是单个字段。

## 方法 `impl`

在 `impl` 块中为结构体定义方法：

```rust
impl User {
    fn greet(&self) {
        println!("Hi, I'm {}", self.name);
    }

    fn birthday(&mut self) {
        self.age += 1;
    }
}
```

- `&self` — 不可变借用，只读访问
- `&mut self` — 可变借用，可修改字段
- 调用：`user.greet()`、`alice.birthday()`

## 关联函数

没有 `self` 的函数是**关联函数**，常用作构造函数，以 `Type::name()` 调用：

```rust
impl User {
    fn new(name: &str, age: u32) -> User {
        User {
            name: String::from(name),
            age,
            active: true,
        }
    }
}

let bob = User::new("Bob", 25);
```

## 元组结构体

字段没有名字、只有类型时，可用元组结构体：

```rust
struct Color(u8, u8, u8);
let red = Color(255, 0, 0);
println!("{}", red.0);
```

适合字段含义清晰、数量少的场景（如 RGB、坐标）。

## 解构

用模式匹配一次性取出字段：

```rust
let User { name, age, .. } = user;
```

`..` 表示忽略其余字段。

## 运行

在项目根目录执行：

```sh
rustc structs.rs
./structs
```

Windows 下运行 `structs.exe`。

预期输出：

```text
=== 定义与创建 ===
Alice is 30 years old, active: true
Hi, I'm Bob (25)
=== 可变 struct ===
Alice is now 31
=== 元组结构体 ===
RGB(255, 0, 0)
=== 解构 ===
destructured: Alice, 31
```

## 小结

| 写法 | 说明 |
|------|------|
| `struct Name { field: Type }` | 定义命名字段的结构体 |
| `Name { field: val }` | 创建实例 |
| `instance.field` | 访问字段 |
| `impl Name { fn m(&self) }` | 定义方法 |
| `Name::new()` | 关联函数（无 `self`） |
| `struct T(u8, u8)` | 元组结构体 |

## 延伸阅读

- [`元组`](/syntax/tuples) — 匿名组合 vs 命名字段
- [`函数`](/syntax/functions) — 方法中的 `fn` 与返回值
- [`所有权和移动`](/syntax/ownership) — 结构体字段的所有权
