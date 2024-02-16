// 런타임 시점에 판단하여 빌림을 반환하는 케이스
// x, y, 반환타입 모두 소멸시점이 명확히 드러나지 않아 컴파일 오류 발생
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");

    let result = longest(&s1, &s2);
    println!("{}와 {}중 더 긴 문자열은 '{}'", s1, s2, result);
}
