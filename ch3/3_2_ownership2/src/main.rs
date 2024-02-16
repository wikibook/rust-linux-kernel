fn main() {
    // 새로운 문자열 변수를 생성
    let s1 = String::from("Hello Rust!");

    // s1을 복사하여 s2에 저장
    let s2 = s1.clone();

    // s1은 여전히 오너쉽을 가지고 있기 때문에 문제없음
    println!("{}", s1);
}
