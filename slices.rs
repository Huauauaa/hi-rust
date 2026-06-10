fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn double_in_place(nums: &mut [i32]) {
    for n in nums.iter_mut() {
        *n *= 2;
    }
}

fn main() {
    println!("=== 数组切片 ===");
    let arr = [10, 20, 30, 40, 50];
    let mid = &arr[1..4];
    println!("mid = {:?}", mid);
    println!("len = {}", mid.len());

    let head = &arr[..3];
    let tail = &arr[3..];
    println!("head = {:?}, tail = {:?}", head, tail);

    println!("=== 字符串切片 ===");
    let s = String::from("hello rust");
    let hello = &s[..5];
    let rust = &s[6..];
    println!("hello = {hello}, rust = {rust}");
    println!("s 仍可用: {s}");

    let word = first_word("hi-rust lang");
    println!("first_word = {word}");

    println!("=== 范围语法 ===");
    let nums = [1, 2, 3, 4, 5];
    println!("全部: {:?}", &nums[..]);
    println!("含末尾: {:?}", &nums[2..=4]);

    println!("=== 作为参数 ===");
    println!("sum = {}", sum(mid));

    println!("=== 可变切片 ===");
    let mut data = [1, 2, 3];
    double_in_place(&mut data);
    println!("data = {:?}", data);
}
