use std::rc::Rc;

struct Person {
    name: String,
    age: i32,
    next: Option<Rc<Person>>, // next는 수정 불가
}

fn main() {
    let mut p1 = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: None,
    });

    let mut p2 = Rc::new(Person {
        name: String::from("Rust"),
        age: 10,
        next: None,
    });

    p1.next = Some(p2.clone()); // 컴파일 오류 발생
}
