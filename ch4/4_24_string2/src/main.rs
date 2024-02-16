fn main() {
    let str = String::from("안녕");
    let idx = 123;

    let s = format!("{} {}", str, idx);
    println!("{}", s);
}
