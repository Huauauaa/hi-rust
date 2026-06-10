struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Text(String),
    Quit,
}

fn main() {
    println!("=== 遮蔽与类型变化 ===");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {spaces}");

    println!("=== 解构元组 ===");
    let pair = (10, 20);
    let (a, b) = pair;
    println!("a = {a}, b = {b}");

    println!("=== 解构 struct ===");
    let p = Point { x: 3, y: 4 };
    let Point { x, y } = p;
    println!("x = {x}, y = {y}");

    let Point { x: px, .. } = Point { x: 7, y: 8 };
    println!("px = {px}");

    println!("=== 解构 enum ===");
    let msg = Message::Text(String::from("hi-rust"));
    if let Message::Text(text) = msg {
        println!("text = {text}");
    }
    let action = match Message::Quit {
        Message::Text(t) => t,
        Message::Quit => String::from("quit"),
    };
    println!("action = {action}");

    println!("=== ref 绑定 ===");
    let value = 42;
    let ref r = value;
    println!("value = {value}, r = {r}");

    println!("=== 忽略值 _ ===");
    let (_unused, used) = (100, 200);
    println!("used = {used}");

    println!("=== ref mut 解构 ===");
    let mut point = Point { x: 1, y: 2 };
    let Point { ref mut x, .. } = point;
    *x += 10;
    println!("point.x = {}", point.x);
}
