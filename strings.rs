fn main() {
    let literal: &str = "hello";
    let mut owned = String::from("Rust");
    owned.push('!');
    owned.push_str(" 🦀");

    let greeting = format!("{literal}, {owned}");

    println!("=== 创建与拼接 ===");
    println!("literal: {literal}");
    println!("owned: {owned}");
    println!("greeting: {greeting}");
    println!("字节长度: {}", greeting.len());
    println!("字符数: {}", greeting.chars().count());

    println!("=== 查询 ===");
    println!("contains Rust: {}", greeting.contains("Rust"));
    println!("starts_with hello: {}", greeting.starts_with("hello"));
    println!("ends_with 🦀: {}", greeting.ends_with("🦀"));
    println!("is_empty: {}", greeting.is_empty());
    println!("find !: {:?}", greeting.find('!'));

    println!("=== 转换 ===");
    println!("to_uppercase: {}", greeting.to_uppercase());
    println!("to_lowercase: {}", greeting.to_lowercase());
    println!("trim: [{}]", "  hi  ".trim());
    println!("replace: {}", greeting.replace("Rust", "rust"));

    println!("=== 分割 ===");
    for part in "a,b,c".split(',') {
        println!("part: {part}");
    }

    println!("=== 修改 ===");
    let mut s = String::from("hello");
    s.insert(5, '!');
    println!("insert: {s}");
    println!("pop: {:?}", s.pop());
    s.push_str(" world");
    println!("after push_str: {s}");
    s.clear();
    println!("after clear, is_empty: {}", s.is_empty());
}
