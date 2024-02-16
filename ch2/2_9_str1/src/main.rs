fn main() {
    // 문자열 리터럴을 사용하여 txt 생성
    let txt: &str = "Hello 러스트!";

    // txt 값 출력
    println!("문자열: {}", txt);

    // 문자열 슬라이싱
    let slice: &str = &txt[0..5];
    println!("슬라이스: {}", slice);
}