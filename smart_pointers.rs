use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(head: i32, tail: List) -> List {
        List::Cons(head, Box::new(tail))
    }
}

fn sum_list(list: &List) -> i32 {
    match list {
        List::Cons(n, tail) => n + sum_list(tail),
        List::Nil => 0,
    }
}

#[derive(Debug)]
struct Owner {
    name: String,
}

fn main() {
    println!("=== Box<T> ===");
    let boxed = Box::new(42);
    println!("boxed = {boxed}");
    let list = List::prepend(3, List::prepend(2, List::prepend(1, List::new())));
    println!("sum = {}", sum_list(&list));

    println!("=== Rc<T> ===");
    let shared = Rc::new(Owner {
        name: String::from("hi-rust"),
    });
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);
    println!("a.name = {}", a.name);
    println!("b.name = {}", b.name);
    println!("strong_count = {}", Rc::strong_count(&shared));

    println!("=== RefCell<T> ===");
    let cell = RefCell::new(10);
    *cell.borrow_mut() += 5;
    println!("cell = {}", cell.borrow());

    println!("=== Rc<RefCell<T>> ===");
    let value = Rc::new(RefCell::new(vec![1, 2, 3]));
    let v1 = Rc::clone(&value);
    let v2 = Rc::clone(&value);
    v1.borrow_mut().push(4);
    v2.borrow_mut().push(5);
    println!("shared vec = {:?}", value.borrow());

    println!("=== Deref ===");
    let s = Box::new(String::from("Rust"));
    println!("len = {}", s.len());
}
