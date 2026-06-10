# 多线程

Rust 标准库 `std::thread` 提供**操作系统线程**。新建线程常用 `thread::spawn`；多个线程共享数据时，用 `Arc` 共享所有权、`Mutex` 保证互斥访问。传给线程的闭包通常需要 `move`。

## 示例代码

源码在仓库根目录的 [`threads.rs`](https://github.com/Huauauaa/hi-rust/blob/main/threads.rs)：

<<< ../../threads.rs{rust}

## 创建线程

```rust
use std::thread;

let handle = thread::spawn(|| {
    42
});

let result = handle.join().unwrap();
```

- `spawn` 返回 `JoinHandle<T>`，线程返回值从 `join()` 取得。
- `join()` 会**阻塞**当前线程，直到子线程结束。
- 子线程内 `panic` 时，`join()` 返回 `Err`。

## `move` 闭包

线程可能与当前函数并发执行，因此闭包必须 `'static`——不能持有栈上借用的引用。把变量移入线程：

```rust
let name = String::from("Rust");
thread::spawn(move || println!("{name}"));
```

详见 [闭包](/syntax/closures) 中的 `move` 说明。

## 共享可变状态

`Arc<T>`（原子引用计数）让多个线程共享所有权；`Mutex<T>` 提供内部可变性并保证同一时刻只有一个线程访问：

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let counter = Arc::clone(&counter);

thread::spawn(move || {
    let mut num = counter.lock().unwrap();
    *num += 1;
});
```

| 类型 | 说明 |
|------|------|
| `Arc<T>` | 多线程安全的共享指针，用 `clone` 增加引用计数 |
| `Mutex<T>` | 互斥锁，`lock()` 返回 `MutexGuard` |
| `lock().unwrap()` | 锁被 poison 时 panic；生产代码可改用 `?` 处理 |

同一变量只应有一个 `Arc` 所有者；传给每个线程前 `Arc::clone(&arc)`，只增加计数，不复制内部数据。`Arc` 是 `Rc` 的多线程版本，详见 [智能指针](/syntax/smart_pointers)。

## `thread::scope`

Rust 1.63+ 提供 `thread::scope`，在 scope 内 spawn 的线程**保证在 scope 结束前 join**，因此闭包可以借用栈上的数据（仍须满足 Send / Sync）：

```rust
thread::scope(|scope| {
    scope.spawn(|| println!("worker"));
});
// 此处所有 scope 内线程已结束
```

适合生命周期受控的短任务；跨线程共享可变数据仍推荐 `Arc<Mutex<T>>`。

## 并行示例

示例中的 `parallel_sum` 把切片分成多块，各线程局部求和后写入共享 `Mutex`。更复杂的并行任务可考虑 [Rayon](https://github.com/rayon-rs/rayon) 等 crate（需 Cargo 项目）。

## 运行

在项目根目录执行：

```sh
rustc threads.rs
./threads
```

Windows 下运行 `threads.exe`。

预期输出：

```text
=== spawn 与 join ===
result = 42
=== move 闭包 ===
Hello from Rust
=== Arc<Mutex<T>> ===
counter = 3
=== thread::scope ===
scope thread 2
scope thread 1
scope joined
=== 并行求和 ===
sum = 21
```

`thread::scope` 内两行打印顺序可能互换，属正常现象。

## 小结

| 写法 | 说明 |
|------|------|
| `thread::spawn` | 新建 OS 线程 |
| `handle.join()` | 等待线程结束并取返回值 |
| `move` 闭包 | 把捕获变量移入线程 |
| `Arc<Mutex<T>>` | 多线程共享可变数据 |
| `thread::scope` | 限定生命周期的 scoped 线程 |

## 延伸阅读

- [`闭包`](/syntax/closures) — `move` 闭包与 `Fn` trait
- [async / await](/syntax/async_await) — 异步 I/O 与并发另一种模型
- [`智能指针`](/syntax/smart_pointers) — `Rc` 与 `Arc` 的关系
- [`所有权和移动`](/syntax/ownership) — 为何线程需要 `'static`
- [The Rust Book — Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
