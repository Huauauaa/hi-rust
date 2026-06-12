use std::fmt;

struct User {
    name: String,
    age: u32,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}

enum Status {
    Active,
    Inactive,
    Banned(String),
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => f.write_str("Active"),
            Status::Inactive => f.write_str("Inactive"),
            Status::Banned(reason) => f.debug_tuple("Banned").field(reason).finish(),
        }
    }
}

struct Account {
    username: String,
    token: String,
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Account")
            .field("username", &self.username)
            .field("token", &"<redacted>")
            .finish()
    }
}

fn main() {
    println!("=== struct Debug ===");
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{user:?}");

    println!("=== enum Debug ===");
    println!("{:?}", Status::Active);
    println!("{:?}", Status::Inactive);
    let status = Status::Banned(String::from("spam"));
    println!("{status:?}");
    if let Status::Banned(reason) = &status {
        println!("reason = {reason}");
    }

    println!("=== 隐藏敏感字段 ===");
    let account = Account {
        username: String::from("alice"),
        token: String::from("secret-token-123"),
    };
    println!("{account:?}, token len = {}", account.token.len());

    println!("=== 美化输出 {{:#?}} ===");
    println!("{user:#?}");
    println!("{status:#?}");
}
