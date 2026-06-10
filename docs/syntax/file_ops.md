# 文件操作

`std::fs` 提供文件与目录的常见操作：创建、读写、复制、重命名、删除，以及查询元数据和遍历目录。路径用 `Path` / `PathBuf` 表示。

## 示例代码

源码在仓库根目录的 [`file_ops.rs`](https://github.com/Huauauaa/hi-rust/blob/main/file_ops.rs)：

<<< ../../file_ops.rs{rust}

## 路径

| 类型 | 说明 |
|------|------|
| `&Path` | 路径切片，类似 `&str` |
| `PathBuf` | 可拥有的路径，类似 `String` |
| `Path::new("a/b")` | 从字符串创建路径 |
| `path.join("c")` | 拼接子路径，返回 `PathBuf` |
| `path.display()` | 格式化输出（跨平台） |

字符串字面量 `"notes.txt"` 可自动转为 `&Path`；需要修改或拼接时用 `PathBuf`。

## 创建目录

| API | 说明 |
|-----|------|
| `fs::create_dir(path)` | 创建单级目录，父目录须已存在 |
| `fs::create_dir_all(path)` | 递归创建，中间目录不存在也会一并创建 |

## 读写文件

| API | 说明 |
|-----|------|
| `fs::write(path, contents)` | 一次性写入（覆盖） |
| `fs::read_to_string(path)` | 读入整个文件 |
| `fs::read(path)` | 读入为 `Vec<u8>` |
| `File::open(path)` | 打开已有文件 |
| `OpenOptions::new().append(true).open(path)` | 以追加模式打开 |

追加写入时，`writeln!(file, "...")` 会在末尾加换行。写入后文件句柄在 `drop` 时自动关闭。

## 元数据

`fs::metadata(path)` 返回 `Metadata`，常用方法：

| 方法 | 说明 |
|------|------|
| `len()` | 文件大小（字节） |
| `is_file()` | 是否为普通文件 |
| `is_dir()` | 是否为目录 |

## 复制、重命名与删除

| API | 说明 |
|-----|------|
| `fs::copy(from, to)` | 复制文件 |
| `fs::rename(from, to)` | 重命名或移动 |
| `fs::remove_file(path)` | 删除文件 |
| `fs::remove_dir(path)` | 删除**空**目录 |

`rename` 在 Unix 上可跨文件系统移动；目标已存在时行为因平台而异，生产代码需额外处理。

## 遍历目录

```rust
for entry in fs::read_dir("dir")? {
    let entry = entry?;
    let name = entry.file_name();
    let is_dir = entry.file_type()?.is_dir();
}
```

`read_dir` 返回的迭代顺序不固定；需要稳定输出时可收集后排序。

## 错误处理

文件操作返回 `io::Result<T>`，与 [错误处理](/syntax/error_handling) 章节一样，可用 `?` 向上传播：

```rust
fn load_config(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}
```

## 运行

在项目根目录执行：

```sh
rustc file_ops.rs
./file_ops
```

Windows 下运行 `file_ops.exe`。

预期输出：

```text
=== 创建目录 ===
created file_ops_demo
=== 写入与追加 ===
line 1
line 2
=== 元数据 ===
size = 14 bytes
is_file = true
=== 复制与重命名 ===
renamed to file_ops_demo/notes_renamed.txt
=== 列出目录 ===
file: notes.txt
file: notes_renamed.txt
=== 清理 ===
removed file_ops_demo
```

## 小结

| 写法 | 说明 |
|------|------|
| `Path::new` / `.join` | 构建路径 |
| `create_dir_all` | 递归建目录 |
| `write` / `read_to_string` | 整文件读写 |
| `OpenOptions` + `append` | 追加写入 |
| `metadata` | 文件信息 |
| `copy` / `rename` / `remove_*` | 复制、移动、删除 |
| `read_dir` | 列出目录内容 |

## 延伸阅读

- [IO](/syntax/io) — 标准输入输出与基础文件读写
- [`字符串`](/syntax/strings) — 文本与字节
- [The Rust Book — Files](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
