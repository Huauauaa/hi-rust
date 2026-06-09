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

    println!("=== 元组结构体 ===");
    let red = Color(255, 0, 0);
    println!("RGB({}, {}, {})", red.0, red.1, red.2);

    println!("=== 解构 ===");
    let User { name, age, .. } = alice;
    println!("destructured: {name}, {age}");
}
