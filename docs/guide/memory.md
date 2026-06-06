# 内存

Rust 在**编译期**保证内存安全：没有垃圾回收，也无需手动 `free`。核心机制是**所有权**、**移动**与**借用**。

## 讲解视频

<video controls width="100%" style="max-width: 960px; border-radius: 8px;">
  <source src="/videos/rust-memory.mp4" type="video/mp4" />
  您的浏览器不支持视频播放。
</video>

视频时长约 30 秒，涵盖以下内容：

| 章节 | 要点 |
|------|------|
| 栈 vs 堆 | 栈存放固定大小数据；堆用于 `String`、`Vec` 等动态分配 |
| 所有权 | 每个值有且仅有一个所有者；离开作用域自动 `drop` |
| Move | 堆数据赋值时转移所有权，原变量不再有效 |
| 借用 | `&T` 共享只读；`&mut T` 同一时刻仅一个可变引用 |

## 与语法章节的联系

- [`定义变量`](/guide/variables) — 绑定与可变性
- [`字符串`](/guide/strings) — `&str` 在栈上引用，`String` 在堆上分配

## 延伸阅读

- [The Rust Book — Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- 仓库中的 HyperFrames 项目：[`rust-memory/`](https://github.com/Huauauaa/hi-rust/tree/main/rust-memory)
