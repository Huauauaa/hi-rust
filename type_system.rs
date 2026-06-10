type UserId = u64;

fn describe_number(n: i32) -> String {
    format!("i32: {n}")
}

fn add_u32(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    println!("=== 标量类型 ===");
    let integer: i32 = -42;
    let unsigned: u32 = 42;
    let float: f64 = 3.14;
    let flag: bool = true;
    let letter: char = '中';
    println!("{integer}, {unsigned}, {float}, {flag}, {letter}");

    println!("=== 类型推断 ===");
    let inferred = 100;
    let explicit: u16 = 100;
    println!("inferred = {inferred}, explicit = {explicit}");
    println!("describe = {}", describe_number(inferred));

    println!("=== 单元类型 ===");
    let unit: () = ();
    println!("unit = {unit:?}");

    println!("=== 类型别名 ===");
    let id: UserId = 1001;
    println!("UserId = {id}");

    println!("=== as 转换 ===");
    let n = 65_i32;
    let byte = n as u8;
    let ch = char::from(byte);
    println!("byte = {byte}, char = {ch}");

    println!("=== parse / into ===");
    let parsed: i32 = "42".parse().expect("number");
    let name: String = "hi-rust".into();
    println!("parsed = {parsed}, name = {name}");

    println!("=== 复合类型 ===");
    let pair: (i32, &str) = (1, "tuple");
    let arr: [i32; 3] = [1, 2, 3];
    println!("pair = {pair:?}, arr = {arr:?}");

    println!("=== 表达式须同一类型 ===");
    let v = if flag { 1_i32 } else { 2 };
    println!("v = {v}");

    println!("=== 不同类型不可混用 ===");
    let a: u32 = 1;
    let b: u32 = 2;
    println!("u32 sum = {}", add_u32(a, b));
}
