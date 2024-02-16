fn main() {
    let mut eng = String::new();
    eng.push_str("hello");
    let jpn = "こんにちは".to_string();
    let kor = String::from("안녕하세요");

    println!("{} {} {}", eng, jpn, kor);
}
