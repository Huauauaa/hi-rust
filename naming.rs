const MAX_CONNECTIONS: u32 = 100;

struct UserProfile {
    display_name: String,
}

enum HttpStatus {
    Ok,
    NotFound,
}

trait Greeter {
    fn greet(&self, user_name: &str);
}

impl Greeter for UserProfile {
    fn greet(&self, user_name: &str) {
        println!("Hello, {user_name}! I'm {}", self.display_name);
    }
}

fn calculate_total_price(unit_price: u32, quantity: u32) -> u32 {
    unit_price * quantity
}

fn status_label(status: HttpStatus) -> &'static str {
    match status {
        HttpStatus::Ok => "OK",
        HttpStatus::NotFound => "404",
    }
}

fn main() {
    let user_name = "alice";
    let mut request_count = 0;
    request_count += 1;

    let profile = UserProfile {
        display_name: String::from("Alice"),
    };

    println!("MAX_CONNECTIONS: {MAX_CONNECTIONS}");
    println!("user: {user_name}, requests: {request_count}");
    profile.greet(user_name);
    println!("total: {}", calculate_total_price(10, 3));
    println!("status: {}", status_label(HttpStatus::Ok));
    println!("404 variant: {}", status_label(HttpStatus::NotFound));
}
