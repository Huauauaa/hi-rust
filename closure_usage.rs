fn pick_if(nums: &[i32], min: i32, test: impl Fn(i32) -> bool) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|n| *n >= min && test(*n))
        .collect()
}

fn run_twice(mut action: impl FnMut()) {
    action();
    action();
}

fn make_adder(base: i32) -> impl Fn(i32) -> i32 {
    move |x| base + x
}

fn main() {
    let nums = vec![3, 8, 1, 9, 4, 6];

    println!("=== 迭代器链 ===");
    let evens: Vec<_> = nums
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * 10)
        .collect();
    let sum: i32 = nums.iter().sum();
    let max = nums.iter().copied().reduce(|a, b| a.max(b)).unwrap();
    println!("evens x10 = {evens:?}, sum = {sum}, max = {max}");

    println!("=== 捕获外部变量 ===");
    let threshold = 5;
    let above: Vec<_> = nums.iter().copied().filter(|n| *n > threshold).collect();
    println!(">{threshold} = {above:?}");

    println!("=== sort_by ===");
    let mut words = vec!["Rust", "hi", "hi-rust"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("by len = {words:?}");

    println!("=== Option / Result 与闭包 ===");
    let doubled = "42".parse::<i32>().ok().map(|n| n * 2);
    let chained = "3"
        .parse::<i32>()
        .ok()
        .and_then(|n| if n > 0 { Some(n * 10) } else { None });
    let fallback = None::<i32>.unwrap_or_else(|| 100);
    println!("doubled = {doubled:?}, chained = {chained:?}, fallback = {fallback}");

    let parsed: Result<i32, _> = "oops".parse();
    let msg = parsed.unwrap_or_else(|e| {
        println!("parse failed: {e}");
        0
    });
    println!("msg = {msg}");

    println!("=== 自定义高阶函数 ===");
    let picked = pick_if(&nums, 4, |n| n % 2 == 0);
    println!("pick_if = {picked:?}");
    run_twice(|| println!("  action"));

    println!("=== 闭包工厂 ===");
    let add5 = make_adder(5);
    let add10 = make_adder(10);
    println!("add5(7) = {}, add10(7) = {}", add5(7), add10(7));

    println!("=== inspect 副作用 ===");
    let collected: Vec<_> = nums
        .iter()
        .copied()
        .inspect(|n| {
            if *n > 5 {
                print!("[{n}] ");
            }
        })
        .collect();
    println!();
    println!("collected = {collected:?}");
}
