# 智能指针

**智能指针**是行为类似指针但附带额外语义的数据结构（如自动释放、引用计数）。Rust 标准库中最常用的是 `Box<T>`、`Rc<T>`、`RefCell<T>`；多线程场景用 `Arc<T>`（见 [多线程](/syntax/threads)）。

## 示例代码

源码在仓库根目录的 [`smart_pointers.rs`](https://github.com/Huauauaa/hi-rust/blob/main/smart_pointers.rs)：

<<< ../../smart_pointers.rs{rust}

## Box\<T\>：堆上单一所有者

`Box<T>` 在堆上分配数据，栈上只存指针；所有权唯一，离开作用域时自动 `drop`：

```rust
let boxed = Box::new(42);
```

常见用途：

- 数据较大，不想在栈上拷贝
- 递归类型（如链表），变体大小须固定，下一节点用 `Box` 间接存储
- trait 对象（`Box<dyn Trait>`，后续可学）

`Box` 实现了 `Deref`，可像引用一样调用内部类型的方法：`boxed.len()` 实际调用 `String` 的 `len`。

## Rc\<T\>：单线程共享所有权

`Rc<T>`（reference counted）允许多个所有者**共享**同一份数据，仅适用于**单线程**：

```rust
use std::rc::Rc;

let shared = Rc::new(data);
let a = Rc::clone(&shared);
let b = Rc::clone(&shared);
```

- `Rc::clone` 只增加计数，不克隆堆上数据。
- 最后一个 `Rc` 被 drop 时，内部数据才释放。
- `Rc::strong_count(&shared)` 查看当前引用数。

`Rc` 只提供**不可变**共享；需要共享可变数据时，配合 `RefCell` 或 `Mutex`。

## RefCell\<T\>：内部可变性

`RefCell<T>` 在**运行时**检查借用规则（而非编译期）：

| 方法 | 说明 |
|------|------|
| `borrow()` | 不可变借用，违反时 panic |
| `borrow_mut()` | 可变借用，违反时 panic |

```rust
let cell = RefCell::new(10);
*cell.borrow_mut() += 5;
```

同一时刻只能有一个可变借用或多个不可变借用，与常规借用规则一致，只是推迟到运行时检查。

## Rc\<RefCell\<T\>\>

单线程下共享且可变数据的常见组合：

```rust
let value = Rc::new(RefCell::new(vec![1, 2, 3]));
let v1 = Rc::clone(&value);
v1.borrow_mut().push(4);
```

多个 `Rc` 指向同一 `RefCell`，通过 `borrow_mut` 修改内部值。

## 与 Arc、Mutex 的关系

| 类型 | 场景 |
|------|------|
| `Box<T>` | 单一所有者，堆分配 |
| `Rc<T>` | 单线程多所有者 |
| `RefCell<T>` | 单线程内部可变性 |
| `Arc<T>` | 多线程多所有者 |
| `Mutex<T>` | 多线程互斥可变访问 |

多线程共享可变状态见 [多线程](/syntax/threads) 中的 `Arc<Mutex<T>>`。

## 运行

在项目根目录执行：

```sh
rustc smart_pointers.rs
./smart_pointers
```

Windows 下运行 `smart_pointers.exe`。

预期输出：

```text
=== Box<T> ===
boxed = 42
sum = 6
=== Rc<T> ===
a.name = hi-rust
b.name = hi-rust
strong_count = 3
=== RefCell<T> ===
cell = 15
=== Rc<RefCell<T>> ===
shared vec = [1, 2, 3, 4, 5]
=== Deref ===
len = 4
```

## 小结

| 类型 | 说明 |
|------|------|
| `Box<T>` | 堆分配，单一所有者 |
| `Rc<T>` | 单线程引用计数共享 |
| `RefCell<T>` | 运行时借用检查的可变内部 |
| `Rc<RefCell<T>>` | 单线程共享可变数据 |
| `Deref` | 智能指针自动解引用调用内部方法 |

## 延伸阅读

- [`所有权和移动`](/syntax/ownership) — 单一所有权模型
- [`借用`](/syntax/borrowing) — 编译期借用规则
- [`多线程`](/syntax/threads) — `Arc` 与 `Mutex`
- [The Rust Book — Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
