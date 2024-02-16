#[derive(Clone)]
struct Student {
    age: i32,
    name: String,
}

fn main() {
    let mut s1 = Student { age: 10, name: String::from("Luna") };
    let s2 = s1.clone(); // s1을 명시적으로 복제하여 s2에 저장

    println!("s1: {} s2: {}", s1.name, s2.name);

    s1.name = String::from("Rust"); // s1의 이름을 변경

    println!("s1: {} s2: {}", s1.name, s2.name);
}
