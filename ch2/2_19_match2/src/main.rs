fn main() {
    let var = 1;
    let ret = match var {
        1 => String::from("하나"),
        2 => String::from("둘"),
        _ => String::from("기타"),
    }; // 세미콜론을 붙여야 합니다.
    println!("ret={}", ret);
}
