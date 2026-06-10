use std::fs;
use std::io::{self, BufRead, Cursor, Write};

fn greet_stdout(name: &str) {
    print!("Hello, ");
    io::stdout().flush().unwrap();
    println!("{name}!");
}

fn parse_age(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.trim().parse()
}

fn read_line_from_reader(reader: &mut impl BufRead) -> io::Result<String> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line)
}

fn main() -> io::Result<()> {
    println!("=== 标准输出 ===");
    greet_stdout("Rust");
    eprintln!("(stderr) 日志：程序已启动");

    println!("=== 标准输入（模拟） ===");
    let mut simulated = Cursor::new("Alice\n30\n");
    let name = read_line_from_reader(&mut simulated)?;
    let age_line = read_line_from_reader(&mut simulated)?;
    let age = parse_age(&age_line).expect("年龄应为数字");
    println!("name = {}", name.trim());
    println!("age = {age}");

    println!("=== 文件读写 ===");
    let path = "io_demo.txt";
    fs::write(path, "hi-rust\nIO demo\n")?;
    let contents = fs::read_to_string(path)?;
    println!("file contents:\n{contents}");
    fs::remove_file(path)?;

    Ok(())
}
