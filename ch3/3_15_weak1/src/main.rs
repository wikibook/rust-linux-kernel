use std::rc::Rc;
use std::cell::RefCell;

struct Person {
    id: i32,
    next: RefCell<Option<Rc<Person>>>,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("p{} Drop!", self.id); // p1과 p2는 drop이 불리지 않음
    }
}

fn main() {
    let mut p1 = Rc::new(Person {
        id: 1,
        next: RefCell::new(None),
    });

    let mut p2 = Rc::new(Person {
        id: 2,
        next: RefCell::new(None),
    });

    let mut p3 = Rc::new(Person {
        id: 3,
        next: RefCell::new(None),
    });

    let mut next = p1.next.borrow_mut();
    *next = Some(p2.clone()); // p1뒤에 p2를 추가
    
    let mut next = p2.next.borrow_mut();
    *next = Some(p1.clone()); // p2뒤에 p1를 추가

    println!("p1 RefCount: {} p2: RefCount: {}", 
        Rc::strong_count(&p1), Rc::strong_count(&p2));

    // p3는 Drop이 불리는데 p1과 p2는 Drop이 불리지 않음
}