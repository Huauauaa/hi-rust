pub mod vegetables;

pub fn describe() -> &'static str {
    "garden module"
}

pub use vegetables::Plant;
