pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn version_info() -> (&'static str, &'static str) {
    ("cargo_demo", env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_works() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }
}
