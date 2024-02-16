// 런타임 시점에 판단하여 빌림을 반환하는 케이스
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
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
