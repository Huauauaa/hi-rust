fn sum(nums: [i32; 5]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

fn main() {
    println!("=== 创建与访问 ===");
    let nums = [10, 20, 30, 40, 50];
    println!("nums[0] = {}", nums[0]);
    println!("nums[4] = {}", nums[4]);
    println!("len = {}", nums.len());

    let zeros: [i32; 5] = [0; 5];
    println!("zeros = {:?}", zeros);

    println!("=== 可变数组 ===");
    let mut scores = [90, 85, 88];
    scores[1] = 92;
    println!("scores = {:?}", scores);

    println!("=== 遍历 ===");
    let days = ["Mon", "Tue", "Wed"];
    for day in days {
        print!("{day} ");
    }
    println!();

    for (i, day) in days.iter().enumerate() {
        println!("days[{i}] = {day}");
    }

    println!("=== 作为参数 ===");
    println!("sum = {}", sum(nums));
}
