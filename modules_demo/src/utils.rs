pub fn greet(name: &str) -> String {
    format!("Hello, {name}")
}

fn internal_helper() -> i32 {
    42
}

pub fn public_answer() -> i32 {
    internal_helper()
}
