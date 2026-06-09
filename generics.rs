fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Pair<T, U> {
    first: T,
    second: U,
}

#[derive(Debug)]
enum Maybe<T> {
    Some(T),
    None,
}

fn main() {
    println!("=== 泛型函数 ===");
    let nums = vec![34, 50, 25, 100, 65];
    println!("largest int = {}", largest(&nums));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char = {}", largest(&chars));

    println!("=== 泛型 struct ===");
    let int_p = Point { x: 5, y: 10 };
    let float_p = Point { x: 1.0, y: 4.0 };
    println!("int: ({}, {}), x = {}", int_p.x, int_p.y, int_p.x());
    println!("float: ({}, {})", float_p.x, float_p.y);

    println!("=== 多个类型参数 ===");
    let pair = Pair {
        first: 1,
        second: "Rust",
    };
    println!("{}, {}", pair.first, pair.second);

    println!("=== 泛型 enum ===");
    let some_int = Maybe::Some(5);
    let some_str = Maybe::Some("hi");
    let none: Maybe<i32> = Maybe::None;
    println!("{some_int:?}, {some_str:?}, {none:?}");
}
