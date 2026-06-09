enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn describe(&self) -> &str {
        match self {
            Message::Quit => "quit",
            Message::Move { .. } => "move",
            Message::Write(_) => "write",
            Message::ChangeColor(..) => "change_color",
        }
    }
}

fn route(kind: IpAddrKind) {
    match kind {
        IpAddrKind::V4 => println!("route: IPv4"),
        IpAddrKind::V6 => println!("route: IPv6"),
    }
}

fn main() {
    println!("=== 简单枚举 ===");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    println!("=== 带数据的变体 ===");
    let msg = Message::Write(String::from("hello"));
    println!("describe: {}", msg.describe());

    let color = Message::ChangeColor(255, 128, 0);
    println!("describe: {}", color.describe());

    let mv = Message::Move { x: 10, y: 20 };
    if let Message::Move { x, y } = mv {
        println!("move to ({x}, {y})");
    }

    println!("=== match 枚举 ===");
    let msg2 = Message::Quit;
    let action = match msg2 {
        Message::Quit => "退出",
        Message::Move { .. } => "移动",
        Message::Write(text) => {
            println!("  写入: {text}");
            "写入"
        }
        Message::ChangeColor(r, g, b) => {
            println!("  颜色: ({r}, {g}, {b})");
            "改色"
        }
    };
    println!("action: {action}");

    println!("=== Option ===");
    let some_num = Some(5);
    let no_num: Option<i32> = None;

    match some_num {
        Some(n) => println!("some_num = {n}"),
        None => println!("some_num is None"),
    }
    match no_num {
        Some(n) => println!("no_num = {n}"),
        None => println!("no_num is None"),
    }
}
