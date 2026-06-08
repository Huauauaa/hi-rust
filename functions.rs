fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn double(x: i32) -> i32 {
    x * 2
}

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn abs_diff(a: i32, b: i32) -> i32 {
    if a >= b {
        return a - b;
    }
    b - a
}

fn print_twice(msg: &str) {
    println!("{msg}");
    println!("{msg}");
}

fn log_status(ok: bool) {
    if ok {
        println!("status: OK");
    } else {
        println!("status: FAIL");
    }
}

fn main() {
    println!("=== 调用函数 ===");
    greet("Rust");
    let sum = add(3, 5);
    println!("3 + 5 = {sum}");

    println!("=== 表达式返回值 ===");
    println!("double(4) = {}", double(4));
    println!("max(7, 3) = {}", max(7, 3));

    println!("=== return 提前返回 ===");
    println!("abs_diff(10, 3) = {}", abs_diff(10, 3));
    println!("abs_diff(3, 10) = {}", abs_diff(3, 10));

    println!("=== 引用参数 ===");
    print_twice("hi-rust");

    println!("=== 无返回值 ===");
    log_status(true);
    log_status(false);
}
