fn take_ownership(s: String) {
    println!("take_ownership: {s}");
}

fn give_ownership() -> String {
    String::from("returned")
}

fn copy_integer(x: i32) {
    println!("copy_integer: {x}");
}

fn main() {
    println!("=== Copy 类型 ===");
    let a = 5;
    let b = a;
    println!("a = {a}, b = {b}");

    println!("=== Move：String ===");
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {s2}");

    println!("=== 作用域与 drop ===");
    {
        let temp = String::from("scope");
        println!("temp = {temp}");
    }
    println!("temp 已离开作用域");

    println!("=== 函数与所有权 ===");
    let s3 = String::from("transfer");
    take_ownership(s3);

    let s4 = give_ownership();
    println!("s4 = {s4}");

    copy_integer(42);

    println!("=== clone ===");
    let original = String::from("hi-rust");
    let cloned = original.clone();
    println!("original = {original}, cloned = {cloned}");
}
