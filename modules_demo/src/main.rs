mod garden;
mod utils;

use garden::Plant;
use utils::greet;

fn main() {
    println!("=== 文件模块 ===");
    println!("{}", garden::describe());
    let plant = garden::vegetables::tomato();
    println!("plant = {}", plant.name);
    println!("label = {}", garden::vegetables::label(&plant));

    println!("=== pub use 重导出 ===");
    let spinach = Plant {
        name: String::from("spinach"),
    };
    println!("plant = {}", spinach.name);

    println!("=== use 路径 ===");
    println!("{}", greet("hi-rust"));

    println!("=== 隐私规则 ===");
    println!("public = {}", utils::public_answer());
}
