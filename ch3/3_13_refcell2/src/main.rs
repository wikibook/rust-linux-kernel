use std::rc::Rc;
use std::cell::RefCell;

struct Person {
    name: String,
    age: i32,
    next: RefCell<Option<Rc<Person>>>, // next는 수정 가능
}

fn main() {
    let mut p1 = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: RefCell::new(None),
    });

    let mut p2 = Rc::new(Person {
        name: String::from("Rust"),
        age: 10,
        next: RefCell::new(None),
    });

    let mut next = p1.next.borrow_mut();
    *next = Some(p2.clone()); // p1뒤에 p2를 추가
}
  