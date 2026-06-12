#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Banned(String),
}

fn main() {
    println!("=== derive Debug ===");
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{:?}, name = {}, age = {}", user, user.name, user.age);
    println!("{user:?}");

    println!("=== 美化输出 {{:#?}} ===");
    println!("{:#?}", Status::Active);
    println!("{:#?}", Status::Inactive);
    let status = Status::Banned(String::from("spam"));
    println!("{status:#?}");
    if let Status::Banned(reason) = &status {
        println!("reason = {reason}");
    }

    println!("=== dbg! 宏 ===");
    let x = 42;
    let y = dbg!(x * 2);
    println!("y = {y}");

    println!("=== 标准库类型 ===");
    let opt: Option<i32> = Some(5);
    let res: Result<i32, &str> = Err("oops");
    println!("{opt:?}, {res:?}");

    println!("=== 元组与 Vec ===");
    let pair = (1, "Rust");
    let nums = vec![1, 2, 3];
    println!("{pair:?}, {nums:?}");
}
