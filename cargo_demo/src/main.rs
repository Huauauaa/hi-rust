use cargo_demo::{greet, version_info};

fn main() {
    println!("=== cargo run ===");
    println!("{}", greet("hi-rust"));

    let (name, version) = version_info();
    println!("crate: {name} v{version}");
}
