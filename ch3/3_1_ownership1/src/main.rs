fn main() {
    let s = String::from("Hello");
    push_str(s); // push_str에 소유권이 이관
    println!("{}", s); // s를 사용하는 순간 컴파일 오류 발생
}

fn push_str(mut s: String) {
    s.push_str(" Rust!");
}
