fn main() {
    let a = 10;
    let b = 3;

    println!("=== 算术 ===");
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);

    println!("=== 比较 ===");
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a > b: {}", a > b);
    println!("a < b: {}", a < b);
    println!("a >= b: {}", a >= b);
    println!("a <= b: {}", a <= b);

    println!("=== 逻辑 ===");
    let t = true;
    let f = false;
    println!("t && f: {}", t && f);
    println!("t || f: {}", t || f);
    println!("!t: {}", !t);

    println!("=== 位运算 ===");
    let x: u8 = 0b1010;
    let y: u8 = 0b1100;
    println!("x & y = {:08b}", x & y);
    println!("x | y = {:08b}", x | y);
    println!("x ^ y = {:08b}", x ^ y);
    println!("!x    = {:08b}", !x);
    println!("x << 1 = {:08b}", x << 1);
    println!("x >> 1 = {:08b}", x >> 1);

    println!("=== 复合赋值 ===");
    let mut n = 5;
    n += 2;
    println!("n += 2 → {n}");
    n -= 1;
    println!("n -= 1 → {n}");
    n *= 3;
    println!("n *= 3 → {n}");
    n /= 2;
    println!("n /= 2 → {n}");
    n %= 4;
    println!("n %= 4 → {n}");
}
