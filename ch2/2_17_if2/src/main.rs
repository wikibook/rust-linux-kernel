fn main() {
    let condition = true;
    let ret = if condition == true {
        String::from("조건이 참 입니다.") // ;을 붙이면 컴파일 오류 발생합니다.
    } else {
        String::from("조건이 거짓 입니다.") // ;을 붙이면 컴파일 오류 발생합니다.
    }; // 여기는 ;을 붙이셔야 합니다.
    println!("ret={}", ret);
}