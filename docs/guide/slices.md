# 切片

**切片**（slice）是对连续元素序列的**引用**，不拥有数据。数组切片类型为 `&[T]`，字符串切片类型为 `&str`。用范围语法 `[start..end]` 从数组或 `String` 上切出一段。

## 示例代码

源码在仓库根目录的 [`slices.rs`](https://github.com/Huauauaa/hi-rust/blob/main/slices.rs)：

<<< ../../slices.rs{rust}

## 数组切片 `&[T]`

对数组取子范围，得到引用类型的切片：

```rust
let arr = [10, 20, 30, 40, 50];
let mid = &arr[1..4]; // [20, 30, 40]，类型 &[i32]
```

- 左闭右开：`[1..4]` 包含下标 1、2、3，不含 4。
- 原数组 `arr` 仍有效，切片只是借用其中一段。
- `.len()` 返回切片长度。

## 字符串切片 `&str`

`String` 和字符串字面量都可以切出 `&str`：

```rust
let s = String::from("hello rust");
let hello = &s[..5];  // "hello"
let rust = &s[6..];   // "rust"
```

字符串切片必须在 **UTF-8 字符边界**上切，否则编译或运行会报错。按空格取第一个单词是常见用法：

```rust
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}
```

## 范围语法

| 写法 | 含义 |
|------|------|
| `[a..b]` | 下标 a 到 b-1 |
| `[a..]` | 从 a 到末尾 |
| `[..b]` | 从头到 b-1 |
| `[..]` | 全部 |
| `[a..=b]` | 下标 a 到 b（含 b） |

## 作为函数参数

切片作为参数时不必关心原数组长度，同一函数可接受任意长度的 `&[i32]` 或 `&str`：

```rust
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
```

这比写死 `[i32; 5]` 更灵活。

## 可变切片 `&mut [T]`

需要修改切片中的元素时，用可变切片：

```rust
fn double_in_place(nums: &mut [i32]) {
    for n in nums.iter_mut() {
        *n *= 2;
    }
}

let mut data = [1, 2, 3];
double_in_place(&mut data);
```

`&str` 不可变，没有 `&mut str`；要改字符串内容需对 `String` 使用 `&mut String` 或 `&mut str` 不适用。

## 运行

在项目根目录执行：

```sh
rustc slices.rs
./slices
```

Windows 下运行 `slices.exe`。

预期输出：

```text
=== 数组切片 ===
mid = [20, 30, 40]
len = 3
head = [10, 20, 30], tail = [40, 50]
=== 字符串切片 ===
hello = hello, rust = rust
s 仍可用: hello rust
first_word = hi-rust
=== 范围语法 ===
全部: [1, 2, 3, 4, 5]
含末尾: [3, 4, 5]
=== 作为参数 ===
sum = 90
=== 可变切片 ===
data = [2, 4, 6]
```

## 小结

| 类型 | 来源 | 说明 |
|------|------|------|
| `&[T]` | 数组、向量 | 借用连续元素 |
| `&str` | `String`、字面量 | 借用 UTF-8 字符串片段 |
| `&mut [T]` | 可变数组 | 可修改元素 |
| `[a..b]` | 范围语法 | 左闭右开 |

## 延伸阅读

- [`借用`](/guide/borrowing) — `&` 与 `&mut`
- [`数组`](/guide/arrays) — 固定长度数组
- [`字符串`](/guide/strings) — `String` 与 `&str`
