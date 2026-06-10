use std::convert::{TryFrom, TryInto};

fn main() {
    println!("=== as 数值转换 ===");
    let big: i32 = 300;
    let small = big as u8;
    let pi = 3.9_f64;
    let int_pi = pi as i32;
    println!("300 as u8 = {small}, 3.9 as i32 = {int_pi}");

    let a: i32 = 10;
    let b: i32 = 3;
    let avg = (a as f64) / (b as f64);
    println!("avg = {avg:.2}");

    println!("=== From / Into ===");
    let owned = String::from("hi-rust");
    let also: String = "hello".into();
    let ch = char::from(65_u8);
    println!("{owned}, {also}, char = {ch}");

    println!("=== TryFrom / TryInto ===");
    match u8::try_from(256_i32) {
        Ok(v) => println!("ok = {v}"),
        Err(e) => println!("err = {e}"),
    }
    let byte = 10_u8;
    let number: i32 = byte.try_into().unwrap();
    println!("u8 -> i32 = {number}");

    println!("=== parse ===");
    let n: i32 = "42".parse().unwrap();
    let f: f64 = "3.14".parse().unwrap();
    let bad: Result<i32, _> = "oops".parse();
    println!("n = {n}, f = {f}, bad = {bad:?}");

    println!("=== to_string ===");
    let text = 100_i32.to_string();
    let flag = true.to_string();
    println!("text = {text}, flag = {flag}");

    println!("=== String 与 &str ===");
    let owned = String::from("Rust");
    let slice: &str = owned.as_str();
    let restored = slice.to_string();
    println!("slice = {slice}, restored = {restored}");

    println!("=== 切片转 Vec ===");
    let arr = [1_i32, 2, 3];
    let vec: Vec<i32> = arr.to_vec();
    println!("vec = {vec:?}");
}
