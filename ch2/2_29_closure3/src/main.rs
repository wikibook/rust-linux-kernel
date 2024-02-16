fn main() {
    let s = String::from("Hello");
    let f = move || { // move클로저는 소유권을 이전합니다.
        println!("s: {}", s); // 여기서 s의 소유권을 가져갑니다.
    };

    f();
    println!("s: {}", s); // 컴파일 오류: s의 소유권이 없습니다.
}