fn main() {
    let score = 85;

    println!("=== if / else if / else ===");
    if score >= 90 {
        println!("等级: A");
    } else if score >= 80 {
        println!("等级: B");
    } else if score >= 60 {
        println!("等级: C");
    } else {
        println!("等级: D");
    }

    println!("=== if 作为表达式 ===");
    let n = 7;
    let parity = if n % 2 == 0 { "偶数" } else { "奇数" };
    println!("{n} 是 {parity}");

    let max = if n > 10 { n } else { 10 };
    println!("max({n}, 10) = {max}");

    println!("=== 嵌套 if ===");
    let logged_in = true;
    let is_admin = false;

    if logged_in {
        if is_admin {
            println!("欢迎，管理员");
        } else {
            println!("欢迎，用户");
        }
    } else {
        println!("请先登录");
    }

    println!("=== 布尔条件 ===");
    let has_ticket = true;
    let age = 16;

    if has_ticket && age >= 18 {
        println!("可以入场");
    } else {
        println!("无法入场");
    }

    println!("=== match 基础 ===");
    let day = 3;
    let weekday = match day {
        1 => "周一",
        2 => "周二",
        3 => "周三",
        4 => "周四",
        5 => "周五",
        6 | 7 => "周末",
        _ => "无效",
    };
    println!("day {day}: {weekday}");

    println!("=== match 范围 ===");
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        60..=79 => "C",
        _ => "D",
    };
    println!("score {score} → {grade}");

    println!("=== match 作为表达式 ===");
    let op = '+';
    let (a, b) = (10, 3);
    let result = match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => 0,
    };
    println!("{a} {op} {b} = {result}");
}
