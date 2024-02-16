use std::io; // std::io 패키지 로드

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1"); // RUST_BACKTRACE 활성화

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // i32타입을 가지는 5개 원소

    println!("숫자를 입력해주세요.");
    let mut read = String::new(); // 입력값을 저장할 문자열 데이터 생성
    io::stdin().read_line(&mut read).unwrap(); // 키보드 입력을 읽습니다.
    let index: i32 = read.trim().parse().unwrap(); // 문자열을 숫자로 변환합니다.

    println!("arr[{}]={}", index, arr[index as usize]);
}
