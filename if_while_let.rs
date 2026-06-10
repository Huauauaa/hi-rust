enum Event {
    Click { x: i32, y: i32 },
    Quit,
}

fn drain(mut items: Vec<i32>) {
    while let Some(n) = items.pop() {
        print!("{n} ");
    }
}

fn main() {
    println!("=== if let 与 Option ===");
    let some = Some(10);
    if let Some(x) = some {
        println!("got {x}");
    }

    let none: Option<i32> = None;
    if let Some(x) = none {
        println!("got {x}");
    } else {
        println!("none branch");
    }

    println!("=== if let 与 enum ===");
    let event = Event::Click { x: 42, y: 7 };
    if let Event::Click { x, y } = event {
        println!("click at ({x}, {y})");
    }
    let quit = Event::Quit;
    if let Event::Quit = quit {
        println!("quit event");
    }

    println!("=== if let 与额外条件 ===");
    let num = Some(4);
    if let Some(x) = num {
        if x % 2 == 0 {
            println!("even {x}");
        }
    }

    println!("=== if let else 表达式 ===");
    let config = Some("prod");
    let mode = if let Some(m) = config { m } else { "default" };
    println!("mode = {mode}");

    let missing: Option<&str> = None;
    let fallback = if let Some(m) = missing { m } else { "default" };
    println!("fallback = {fallback}");

    println!("=== while let ===");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("pop {top} ");
    }
    println!();

    drain(vec![4, 5, 6]);
    println!();

    println!("=== while let 与迭代器 ===");
    let mut nums = vec![10, 20, 30].into_iter();
    while let Some(n) = nums.next() {
        print!("{n} ");
    }
    println!();

    println!("=== let else ===");
    let Some(value) = Some(99) else {
        panic!("expected Some");
    };
    println!("value = {value}");
}
