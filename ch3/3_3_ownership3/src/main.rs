fn main() {
    let s = String::from("Hello");
    let s = push_str(s); // push_str에 ownership을 전달하고, 쉐도잉방식으로 s를 획득
    println!("{}", s);
}

fn push_str(mut s: String) -> String {
    s.push_str(" Rust!");
    s
}
