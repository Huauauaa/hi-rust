use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn parallel_sum(nums: &[i32]) -> i32 {
    let total = Arc::new(Mutex::new(0));
    let chunk_size = (nums.len() + 1) / 2;
    let mut handles = vec![];

    for chunk in nums.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let total = Arc::clone(&total);
        handles.push(thread::spawn(move || {
            let partial: i32 = chunk.iter().sum();
            let mut guard = total.lock().unwrap();
            *guard += partial;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *total.lock().unwrap();
    result
}

fn main() {
    println!("=== spawn 与 join ===");
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        42
    });
    println!("result = {}", handle.join().unwrap());

    println!("=== move 闭包 ===");
    let name = String::from("Rust");
    thread::spawn(move || println!("Hello from {name}"))
        .join()
        .unwrap();

    println!("=== Arc<Mutex<T>> ===");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter = {}", *counter.lock().unwrap());

    println!("=== thread::scope ===");
    thread::scope(|scope| {
        scope.spawn(|| println!("scope thread 1"));
        scope.spawn(|| println!("scope thread 2"));
    });
    println!("scope joined");

    println!("=== 并行求和 ===");
    let nums = vec![1, 2, 3, 4, 5, 6];
    println!("sum = {}", parallel_sum(&nums));
}
