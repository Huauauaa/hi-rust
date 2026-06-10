use std::collections::{HashMap, HashSet};

fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    println!("=== Vec 创建与增删 ===");
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    nums.push(5);
    println!("nums = {:?}", nums);
    println!("pop = {:?}", nums.pop());
    println!("nums = {:?}", nums);

    let mut names = Vec::new();
    names.push(String::from("Rust"));
    names.push(String::from("hi-rust"));
    println!("names = {:?}", names);

    println!("=== Vec 遍历 ===");
    for (i, n) in nums.iter().enumerate() {
        println!("nums[{i}] = {n}");
    }
    println!("sum = {}", sum(&nums));

    println!("=== HashMap ===");
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    println!("Alice = {:?}", scores.get("Alice"));

    scores.entry("Carol").or_insert(88);
    if let Some(score) = scores.get_mut("Alice") {
        *score += 5;
    }
    for (name, score) in &scores {
        println!("{name}: {score}");
    }

    println!("=== HashSet ===");
    let mut tags = HashSet::new();
    tags.insert("rust");
    tags.insert("docs");
    tags.insert("rust");
    println!("len = {}", tags.len());
    println!("contains docs: {}", tags.contains("docs"));
    for tag in &tags {
        print!("{tag} ");
    }
    println!();
}
