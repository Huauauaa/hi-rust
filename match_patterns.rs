enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("=== 字面量与 _ ===");
    let n = 3;
    let label = match n {
        1 => "one",
        2 | 3 => "two or three",
        _ => "other",
    };
    println!("n = {n} → {label}");

    println!("=== 范围与 @ 绑定 ===");
    let score = 95;
    let grade = match score {
        n @ 90..=100 => format!("A ({n})"),
        80..=89 => String::from("B"),
        _ => String::from("C"),
    };
    println!("score {score} → {grade}");

    println!("=== 解构 enum ===");
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to ({x}, {y})"),
        Message::Write(text) => println!("write: {text}"),
        Message::ChangeColor(r, g, b) => println!("color: ({r}, {g}, {b})"),
    }
    let color = Message::ChangeColor(255, 128, 0);
    if let Message::ChangeColor(r, g, b) = color {
        println!("rgb = ({r}, {g}, {b})");
    }
    let _quit = Message::Quit;

    println!("=== 解构 struct 与元组 ===");
    let p = Point { x: 1, y: 2 };
    match p {
        Point { x: 0, y } => println!("on y-axis at {y}"),
        Point { x, y: 0 } => println!("on x-axis at {x}"),
        Point { x, y } => println!("point ({x}, {y})"),
    }

    let pair = (0, 5);
    match pair {
        (0, y) => println!("pair y = {y}"),
        (x, 0) => println!("pair x = {x}"),
        (x, y) => println!("pair ({x}, {y})"),
    }

    println!("=== match 守卫 ===");
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("even {x}"),
        Some(x) => println!("odd {x}"),
        None => println!("none"),
    }

    println!("=== if let ===");
    let msg2 = Message::Write(String::from("hi-rust"));
    if let Message::Write(text) = msg2 {
        println!("if let: {text}");
    }

    println!("=== ref 匹配 ===");
    let value = 42;
    match value {
        ref r => println!("ref = {r}, value still = {value}"),
    }
}
