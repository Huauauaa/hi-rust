use std::time::Instant;
use tokio::time::{sleep, Duration};

async fn fetch_label(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} ready")
}

async fn run_sequential() {
    let a = fetch_label("A", 50).await;
    let b = fetch_label("B", 50).await;
    println!("sequential: {a}, {b}");
}

async fn run_concurrent() {
    let start = Instant::now();
    let (a, b) = tokio::join!(
        fetch_label("A", 50),
        fetch_label("B", 50),
    );
    println!(
        "concurrent: {a}, {b} in {} ms",
        start.elapsed().as_millis()
    );
}

#[tokio::main]
async fn main() {
    println!("=== async fn 与 .await ===");
    let msg = fetch_label("hi-rust", 10).await;
    println!("{msg}");

    println!("=== 顺序 .await ===");
    run_sequential().await;

    println!("=== 并发 tokio::join! ===");
    run_concurrent().await;

    println!("=== tokio::spawn ===");
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(20)).await;
        42
    });
    let n = handle.await.unwrap();
    println!("spawn result = {n}");
}
