fn main() {
    let ret = add(1, 2);
    println!("1+2={}", ret);
}

fn add(x: i32, y: i32) -> i32 {
    x + y // 세미콜론이 없습니다
}