use std::fs;
use std::io;
use std::num::ParseIntError;

fn parse_age(input: &str) -> Result<u32, ParseIntError> {
    input.trim().parse()
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}

fn first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}

fn load_demo_file() -> Result<String, io::Error> {
    let path = "error_demo.txt";
    fs::write(path, "hi-rust\n")?;
    let text = fs::read_to_string(path)?;
    fs::remove_file(path)?;
    Ok(text)
}

fn run_with_question_mark() -> Result<u32, ParseIntError> {
    let n = "42".parse::<u32>()?;
    Ok(n * 2)
}

fn main() {
    println!("=== Option ===");
    let maybe = Some(10);
    let none: Option<i32> = None;
    println!("unwrap_or = {}", maybe.unwrap_or(0));
    println!("map = {:?}", maybe.map(|n| n * 2));
    println!("and_then = {:?}", maybe.and_then(|n| if n > 0 { Some(n) } else { None }));
    if let Some(n) = maybe {
        println!("if let Some = {n}");
    }
    if first_word("hello world").is_some() {
        println!("first_word = {}", first_word("hello world").unwrap());
    }
    println!("none is_none = {}", none.is_none());

    println!("=== Result 与 match ===");
    match parse_age("30") {
        Ok(age) => println!("age = {age}"),
        Err(e) => println!("parse error: {e}"),
    }
    match parse_age("oops") {
        Ok(age) => println!("age = {age}"),
        Err(e) => println!("parse error: {e}"),
    }

    println!("=== ? 运算符 ===");
    match run_with_question_mark() {
        Ok(n) => println!("? result = {n}"),
        Err(e) => println!("? error: {e}"),
    }

    println!("=== unwrap_or / expect ===");
    let n = parse_age("not-a-number").unwrap_or(0);
    println!("unwrap_or default = {n}");
    let ok = parse_age("7").expect("valid age");
    println!("expect ok = {ok}");

    println!("=== 自定义错误信息 ===");
    match divide(10.0, 2.0) {
        Ok(v) => println!("10 / 2 = {v}"),
        Err(msg) => println!("error: {msg}"),
    }
    match divide(10.0, 0.0) {
        Ok(v) => println!("10 / 0 = {v}"),
        Err(msg) => println!("error: {msg}"),
    }

    println!("=== IO 错误 ===");
    match load_demo_file() {
        Ok(text) => print!("file text: {text}"),
        Err(e) => println!("io error: {e}"),
    }

    println!("=== map_err ===");
    let result = parse_age("abc").map_err(|e| format!("invalid age: {e}"));
    match result {
        Ok(age) => println!("age = {age}"),
        Err(msg) => println!("{msg}"),
    }
}
