use std::rc::Rc;

struct Person {
    name: String,
    age: i32,
    next: Option<Rc<Person>>,
}

fn main() {
    let p1 = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: None,
    });

    let p2 = Rc::new(Person {
        name: String::from("Rust"),
        age: 28,
        next: Some(p1.clone()), // Rust의 다음 노드를 Luna로 설정
    });

    print!("{} -> ", p2.name);
    // p2의 다음 노드를 출력
    match &p2.next {
        Some(p) => {
            println!("{}", p.name);
        },
        None => {},
    };
}
