use std::rc::Rc;
use std::cell::RefCell;

struct Person {
    name: String,
    age: i32,
    next: RefCell<Option<Rc<Person>>>,
}

fn push_back(tail: Rc<Person>, name: String, age: i32) -> Rc<Person> {
    let p = Rc::new(Person {
        name: name,
        age: age,
        next: RefCell::new(None)
    });

    let mut next = tail.next.borrow_mut();
    *next = Some(p.clone());

    p
}

fn main() {
    let mut head = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: RefCell::new(None),
    });

    let tail = push_back(head.clone(), String::from("Rust"), 10);
    let tail = push_back(tail.clone(), String::from("Wikibooks"), 20);

    let mut current = head.clone();
    loop {
        print!("{} -> ", current.name);
        let t = current.clone();
        current = match &(*(t.next.borrow_mut())) {
            Some(p) => p,
            None => break,
        }.clone();
    }
}