use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} yrs)", self.name, self.age)
    }
}

fn main() {
    println!("=== Display 与 {{}} ===");
    let p = Point { x: 3, y: 4 };
    println!("point = {p}");
    println!("{}", p);

    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    println!("user = {user}");

    println!("=== format! 与 to_string ===");
    let text = format!("hello, {user}");
    let owned = user.to_string();
    println!("{text}");
    println!("{owned}");

    println!("=== 标准库类型 ===");
    let s = String::from("Rust");
    let slice: &str = "hi-rust";
    let n = 42_i32;
    println!("{s}, {slice}, {n}");

    println!("=== Display 与 Debug 对比 ===");
    println!("Display: {user}");
    println!("Debug:   {user:?}");
}
