fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self) {
        println!("excerpt: {}", self.part);
    }
}

fn main() {
    println!("=== 函数中的生命周期 ===");
    let s1 = String::from("long string");
    {
        let s2 = String::from("short");
        let result = longest(s1.as_str(), s2.as_str());
        println!("longest = {result}");
    }

    println!("=== struct 中的生命周期 ===");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = Excerpt {
        part: first_sentence,
    };
    excerpt.announce();
    println!("novel still valid: {}...", &novel[..20]);

    println!("=== 'static ===");
    let s: &'static str = "I live for the entire program";
    println!("{s}");
}
