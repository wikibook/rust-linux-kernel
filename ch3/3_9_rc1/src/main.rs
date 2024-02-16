use std::rc::Rc;

struct Person {
    age: i32,
}

fn main() {
    // person을 공유 객체로 생성
    let person = Rc::new(Person { age: 10 });

    // person복제
    let p1 = person.clone();
    println!("person: {} p1: {}", person.age, p1.age);
    println!("RefCount: {}", Rc::strong_count(&person));
    
    // person복제
    let p2 = person.clone();
    println!("RefCount: {}", Rc::strong_count(&person));

    {
        // person복제
        let p3 = person.clone();
        println!("RefCount: {}", Rc::strong_count(&person));

        // p3소멸
    }

    println!("RefCount: {}", Rc::strong_count(&person));
}
