# ref 解构有什么用？

**`ref` 解构让你在模式匹配时绑定引用，而不是拿走（移动）原值。** 需要「拆开看里面，但原容器还要继续用」或「只改某个字段、不想重建整个 struct」时，`ref` / `ref mut` 就派上用场。

基础语法见 [变量绑定](/syntax/bindings) 中的 `ref` 与 `ref mut` 一节；可运行示例在 [`bindings.rs`](https://github.com/Huauauaa/hi-rust/blob/main/bindings.rs)。

## 普通解构会移动

默认解构按**值**绑定：字段被移出后，原 struct / 元组往往就不能再用了。

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 1, y: 2 };
let Point { x, y } = p;
// p 已被移动，不能再访问 p.x
println!("{x}, {y}");
```

对 `String`、`Vec` 这类非 `Copy` 类型更是如此——解构即转移所有权。

## ref 解构：借用字段，保留原值

模式里加 `ref`，绑定的是**不可变引用**，原值仍归你所有：

```rust
let p = Point { x: 1, y: 2 };
let Point { ref x, ref y } = p;
// x: &i32, y: &i32；p 仍完整可用
println!("{} {} | p.x = {}", x, y, p.x);
```

元组同理：

```rust
let pair = (String::from("hello"), 42);
let (ref s, n) = pair;
// s 是 &String，n 是 i32（Copy）；pair 仍可用
println!("{s}, {n}, {}", pair.0);
```

**用途**：解构的同时还要把原容器传给别的函数、或稍后继续读写字段。

## ref mut 解构：原地修改字段

需要改 struct 里某个字段、又不想手动写 `point.x += 1` 时，可以用 `ref mut` 拿到可变引用：

```rust
let mut point = Point { x: 1, y: 2 };
let Point { ref mut x, .. } = point;
*x += 10;
println!("{}", point.x);  // 11
```

等价于 `let x = &mut point.x`，但在模式里一次写出「解构 + 借可变引用」，适合只动部分字段的场景。

## 和 match / if let 的关系

在 `match` 里对 enum 解构时，`ref` 同样表示「绑定引用而非移动内部数据」：

```rust
enum Message {
    Text(String),
    Quit,
}

let msg = Message::Text(String::from("hi-rust"));

match &msg {
    Message::Text(ref text) => println!("{text}"),  // text: &String
    Message::Quit => {}
}
// msg 仍可用
```

现代写法里，更常见的是**直接匹配引用** `match &msg`，分支里自动得到 `&String`，不一定非写 `ref`。但理解 `ref` 有助于读旧代码，以及在 `let` 解构 struct / 元组时精确控制借用。

## 什么时候用？

| 场景 | 写法 |
|------|------|
| 解构后还要用原 struct / 元组 | `let Point { ref x, .. } = p` |
| 只改一个字段，保留整个实例 | `let Point { ref mut x, .. } = p` |
| 解构非 Copy 字段，避免移动 | `let (ref s, n) = pair` |
| enum 内数据只读、外层还要用 | `match &msg { Message::Text(ref t) => ... }` |

## 和直接写 `&` 的区别

| | `ref` 在模式里 | 手动取引用 |
|--|----------------|------------|
| 写法 | `let Point { ref x } = p` | `let x = &p.x` |
| 适用 | 解构时顺带绑定引用 | 已知变量名、单字段访问 |
| 效果 | 绑定 `&T` | 同样是 `&T` |

语义等价，选哪种看是否已经在做模式解构。`ref mut` 则对应 `&mut field`。

## 小结

- 普通解构 → **移动**字段，原值可能失效。
- `ref` 解构 → 绑定 **&T**，原值保留。
- `ref mut` 解构 → 绑定 **&mut T**，可原地改字段。
- 核心用途：**不转移所有权的前提下拆开数据**，或**只借部分字段做修改**。

## 延伸阅读

- [变量绑定](/syntax/bindings) — `ref`、`ref mut` 与完整示例
- [借用](/syntax/borrowing) — `&` / `&mut` 规则
- [所有权和移动](/syntax/ownership) — 何时发生 move
