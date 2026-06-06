fn main() {
    let literal: &str = "hello";
    let mut owned = String::from("Rust");
    owned.push('!');
    owned.push_str(" 🦀");

    let greeting = format!("{literal}, {owned}");

    println!("literal: {literal}");
    println!("owned: {owned}");
    println!("greeting: {greeting}");
    println!("字节长度: {}", greeting.len());
    println!("字符数: {}", greeting.chars().count());
}
