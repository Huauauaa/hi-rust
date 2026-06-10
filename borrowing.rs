fn len(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn append_exclaim(s: &mut String) {
    s.push('!');
}

fn main() {
    println!("=== 不可变借用 &T ===");
    let text = String::from("hi-rust");
    let n = len(&text);
    println!("len = {n}, text 仍可用: {text}");

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("first word: {word}, s: {s}");

    println!("=== 多个不可变借用 ===");
    let x = 10;
    let r1 = &x;
    let r2 = &x;
    println!("r1 = {r1}, r2 = {r2}, x = {x}");

    println!("=== 可变借用 &mut T ===");
    let mut msg = String::from("Rust");
    append_exclaim(&mut msg);
    println!("msg = {msg}");

    msg.push_str(" 🦀");
    println!("msg = {msg}");

    println!("=== 切片引用 ===");
    let data = [1, 2, 3, 4, 5];
    let slice = &data[1..4];
    println!("slice = {:?}", slice);
}
