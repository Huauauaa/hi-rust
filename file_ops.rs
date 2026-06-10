use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

fn list_dir(dir: &Path) -> io::Result<Vec<String>> {
    let mut names: Vec<String> = fs::read_dir(dir)?
        .map(|entry| {
            let entry = entry?;
            let kind = if entry.file_type()?.is_dir() {
                "dir"
            } else {
                "file"
            };
            Ok(format!("{kind}: {}", entry.file_name().to_string_lossy()))
        })
        .collect::<io::Result<_>>()?;
    names.sort();
    Ok(names)
}

fn main() -> io::Result<()> {
    let dir = Path::new("file_ops_demo");

    println!("=== 创建目录 ===");
    fs::create_dir_all(dir)?;
    println!("created {}", dir.display());

    println!("=== 写入与追加 ===");
    let file_path = dir.join("notes.txt");
    fs::write(&file_path, "line 1\n")?;
    let mut file = OpenOptions::new().append(true).open(&file_path)?;
    writeln!(file, "line 2")?;
    let contents = fs::read_to_string(&file_path)?;
    print!("{contents}");

    println!("=== 元数据 ===");
    let meta = fs::metadata(&file_path)?;
    println!("size = {} bytes", meta.len());
    println!("is_file = {}", meta.is_file());

    println!("=== 复制与重命名 ===");
    let copy_path = dir.join("notes_copy.txt");
    fs::copy(&file_path, &copy_path)?;
    let renamed = dir.join("notes_renamed.txt");
    fs::rename(&copy_path, &renamed)?;
    println!("renamed to {}", renamed.display());

    println!("=== 列出目录 ===");
    for line in list_dir(dir)? {
        println!("{line}");
    }

    println!("=== 清理 ===");
    fs::remove_file(&file_path)?;
    fs::remove_file(&renamed)?;
    fs::remove_dir(dir)?;
    println!("removed {}", dir.display());

    Ok(())
}
