fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    println!("=== iter 与 for ===");
    for n in &nums {
        print!("{n} ");
    }
    println!();

    println!("=== iter_mut ===");
    let mut vals = vec![1, 2, 3];
    for v in vals.iter_mut() {
        *v *= 2;
    }
    println!("doubled = {vals:?}");

    println!("=== map 与 filter ===");
    let evens: Vec<i32> = nums
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * 10)
        .collect();
    println!("evens x10 = {evens:?}");

    println!("=== enumerate 与 zip ===");
    let tags = ["a", "b", "c"];
    for (i, n) in nums.iter().take(3).enumerate() {
        println!("{i}: {n}");
    }
    for (n, tag) in nums.iter().zip(tags.iter()) {
        println!("{n}-{tag}");
    }

    println!("=== fold 与 sum ===");
    let sum: i32 = nums.iter().sum();
    let product = nums.iter().fold(1, |acc, &n| acc * n);
    println!("sum = {sum}");
    println!("product = {product}");

    println!("=== take / skip / chain ===");
    let part: Vec<_> = nums.iter().skip(1).take(3).cloned().collect();
    println!("skip 1 take 3 = {part:?}");
    let merged: Vec<_> = [1, 2].iter().chain([3, 4].iter()).cloned().collect();
    println!("chain = {merged:?}");

    println!("=== any / all / find ===");
    println!("any > 4: {}", nums.iter().any(|&n| n > 4));
    println!("all > 0: {}", nums.iter().all(|&n| n > 0));
    println!("find == 3: {:?}", nums.iter().find(|&&n| n == 3));

    println!("=== into_iter 消耗所有权 ===");
    let words = vec![String::from("Rust"), String::from("hi-rust")];
    for w in words {
        println!("{w}");
    }
}
