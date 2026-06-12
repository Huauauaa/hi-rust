struct User {
    name: String,
    age: u32,
    active: bool,
}

impl User {
    fn new(name: &str, age: u32) -> User {
        User {
            name: String::from(name),
            age,
            active: true,
        }
    }

    fn greet(&self) {
        println!("Hi, I'm {} ({})", self.name, self.age);
    }

    fn birthday(&mut self) {
        self.age += 1;
    }
}

struct UserProfile {
    name: String,
    age: u32,
    email: Option<String>,
}

struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: String::from("localhost"),
            port: 8080,
            debug: false,
        }
    }
}

struct Color(u8, u8, u8);

fn main() {
    println!("=== 定义与创建 ===");
    let user = User {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };
    println!("{} is {} years old, active: {}", user.name, user.age, user.active);

    let bob = User::new("Bob", 25);
    bob.greet();

    println!("=== 可变 struct ===");
    let mut alice = User::new("Alice", 30);
    alice.birthday();
    println!("{} is now {}", alice.name, alice.age);

    println!("=== 可选字段 ===");
    let with_email = UserProfile {
        name: String::from("Alice"),
        age: 30,
        email: Some(String::from("alice@example.com")),
    };
    let no_email = UserProfile {
        name: String::from("Bob"),
        age: 25,
        email: None,
    };
    println!(
        "{} ({}) email = {:?}, {} ({}) email = {:?}",
        with_email.name,
        with_email.age,
        with_email.email,
        no_email.name,
        no_email.age,
        no_email.email
    );
    if let Some(email) = &with_email.email {
        println!("contact: {email}");
    }

    println!("=== 默认值 ===");
    let cfg = Config {
        port: 3000,
        ..Default::default()
    };
    println!("host = {}, port = {}, debug = {}", cfg.host, cfg.port, cfg.debug);

    println!("=== 元组结构体 ===");
    let red = Color(255, 0, 0);
    println!("RGB({}, {}, {})", red.0, red.1, red.2);

    println!("=== 解构 ===");
    let User { name, age, .. } = alice;
    println!("destructured: {name}, {age}");
}
