fn main() {
    println!("=== loop + break ===");
    let mut count = 0;
    loop {
        count += 1;
        if count >= 3 {
            break;
        }
    }
    println!("count = {count}");

    println!("=== loop 返回值 ===");
    let mut n = 0;
    let doubled = loop {
        n += 1;
        if n == 5 {
            break n * 2;
        }
    };
    println!("doubled = {doubled}");

    println!("=== while ===");
    let mut i = 0;
    while i < 3 {
        print!("{i} ");
        i += 1;
    }
    println!();

    println!("=== for 范围 ===");
    for j in 0..3 {
        print!("{j} ");
    }
    println!();
    for k in 1..=3 {
        print!("{k} ");
    }
    println!();

    println!("=== for 遍历数组 ===");
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits {
        println!("{fruit}");
    }

    println!("=== for 带索引 ===");
    for (idx, fruit) in fruits.iter().enumerate() {
        println!("{idx}: {fruit}");
    }

    println!("=== break / continue ===");
    for num in 1..=6 {
        if num % 2 == 0 {
            continue;
        }
        if num > 5 {
            break;
        }
        print!("{num} ");
    }
    println!();
}
