fn swap(pair: (i32, i32)) -> (i32, i32) {
    (pair.1, pair.0)
}

fn main() {
    println!("=== 创建与访问 ===");
    let point = (3, 4);
    println!("point: ({}, {})", point.0, point.1);

    let mixed = ("Rust", 2024, true);
    println!("mixed: {}, {}, {}", mixed.0, mixed.1, mixed.2);

    println!("=== 解构 ===");
    let (x, y) = point;
    println!("x = {x}, y = {y}");

    let (lang, year, stable) = mixed;
    println!("lang = {lang}, year = {year}, stable = {stable}");

    println!("=== 作为返回值 ===");
    let swapped = swap((10, 20));
    println!("swap(10, 20) = ({}, {})", swapped.0, swapped.1);

    println!("=== 单元类型 () ===");
    let unit = ();
    let _ = unit;
    println!("空元组 () 是单元类型，表示没有有意义的值");
}
