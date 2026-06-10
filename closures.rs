fn apply_filter(nums: &[i32], mut pred: impl FnMut(i32) -> bool) -> Vec<i32> {
    nums.iter().copied().filter(|n| pred(*n)).collect()
}

fn call_twice(mut f: impl FnMut()) {
    f();
    f();
}

fn main() {
    println!("=== 基本语法 ===");
    let add_one = |x| x + 1;
    let multiply = |a, b| a * b;
    println!("add_one(5) = {}", add_one(5));
    println!("multiply(3, 4) = {}", multiply(3, 4));

    println!("=== 捕获环境 ===");
    let factor = 10;
    let scale = |n| n * factor;
    println!("scale(7) = {}", scale(7));

    println!("=== move 闭包 ===");
    let s = String::from("hi-rust");
    let greet = move || println!("Hello, {s}!");
    greet();

    println!("=== 作为参数 ===");
    let nums = vec![1, 2, 3, 4, 5];
    let evens = apply_filter(&nums, |n| n % 2 == 0);
    println!("evens = {evens:?}");

    println!("=== FnMut ===");
    let mut count = 0;
    let mut bump = || {
        count += 1;
        count
    };
    println!("bump = {}", bump());
    println!("bump = {}", bump());

    println!("=== FnOnce ===");
    let name = String::from("Rust");
    let take_name = move || name;
    let owned = take_name();
    println!("owned = {owned}");

    println!("=== 与迭代器 ===");
    let doubled: Vec<_> = nums.iter().map(|&n| n * 2).collect();
    println!("doubled = {doubled:?}");

    println!("=== 高阶用法 ===");
    call_twice(|| println!("tick"));
}
