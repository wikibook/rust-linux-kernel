fn main() {
    let mut s = String::from("Hello");

    // s의 소유권을 push_str에 대여
    push_str(&mut s);

    // s는 소유권을 유지하고 있기에 정상 동작
    println!("{}", s);
}

fn push_str(s: &mut String) {
    s.push_str(" Rust!");
}
