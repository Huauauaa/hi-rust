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

`new` 里未传入的字段可在函数体内设默认值，例如 `active: true`。

## 可选字段

Rust **没有**「非必填字段」语法——创建实例时每个字段都必须赋值。字段可能不存在时，用 `Option<T>` 表示「有值 / 无值」：

```rust
struct UserProfile {
    name: String,
    age: u32,
    email: Option<String>,  // Some(...) 或 None
}

let with_email = UserProfile {
    name: String::from("Alice"),
    age: 30,
    email: Some(String::from("alice@example.com")),
};

let no_email = UserProfile {
    name: String::from("Bob"),
    age: 25,
    email: None,
};
```

读取时用 `if let`、`match` 或 `unwrap_or` 处理 `None`，见 [错误处理](/syntax/error_handling) 中的 `Option`。

## 默认值

Rust **不能**在 struct 定义里直接写字段默认值（没有 `active: bool = true` 这种写法）。常见做法：

**构造函数** — 必填参数传入，其余在 `new` 里设默认（见上文 `User::new` 中的 `active: true`）。

**`Default` trait + `..` 语法** — 适合多数字段有固定默认值的场景：

```rust
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: String::from("localhost"),
            port: 8080,
            debug: false,
        }
    }
}

// 只改 port，其余用 default
let cfg = Config {
    port: 3000,
    ..Default::default()
};
```

带 `String` 的 struct 通常需手写 `impl Default`；若字段全是 `bool`、`u32`、`Option<T>` 等有标准默认值的类型，可 `#[derive(Default)]` 后直接 `Config::default()`。

| 场景 | 推荐 |
|------|------|
| 字段可能真的没有值 | `Option<T>` |
| 有固定默认值，创建时偶尔覆盖 | `Default` + `..Default::default()` |
| 必填少、默认简单 | 构造函数 `new()` |
| 可选字段很多 | Builder 模式（链式 `with_xxx()`） |

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
=== 可选字段 ===
Alice (30) email = Some("alice@example.com"), Bob (25) email = None
contact: alice@example.com
=== 默认值 ===
host = localhost, port = 3000, debug = false
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
| `Option<T>` | 可选字段，值为 `Some` 或 `None` |
| `Default` + `..Default::default()` | 创建时只覆盖部分字段 |
| `struct T(u8, u8)` | 元组结构体 |

## 延伸阅读

- [`元组`](/syntax/tuples) — 匿名组合 vs 命名字段
- [`函数`](/syntax/functions) — 方法中的 `fn` 与返回值
- [`Debug`](/syntax/debug) — `#[derive(Debug)]` 与 `{:?}` 打印
- [`Display`](/syntax/display) — `{}` 与用户可读输出
- [`错误处理`](/syntax/error_handling) — `Option<T>` 的用法
- [`所有权和移动`](/syntax/ownership) — 结构体字段的所有权
