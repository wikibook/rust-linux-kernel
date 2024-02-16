use std::rc::Rc;

struct Person {
    name: String,
    age: i32,
    next: Option<Rc<Person>>,
}

fn push_front(head: Rc<Person>, name: String, age: i32) -> Rc<Person> {
    let p = Rc::new(Person {
        name: name,
        age: age,
        next: Some(head.clone())
    });

    p.clone()
}

fn main() {
    let head = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: None,
    });

    let head = push_front(head, String::from("Rust"), 10);
    let head = push_front(head, String::from("Wikibooks"), 20);

    let mut current = head.clone();
    loop {
        print!("{} -> ", current.name);
        current = match &current.next {
            Some(p) => p,
            None => break,
        }.clone();
    }
}
