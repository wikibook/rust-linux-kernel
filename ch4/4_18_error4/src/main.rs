fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("0으로 나눌 수 없습니다.")
    }

    a / b
}

fn main() {
    let ret = div(1, 0);
    println!("ret: {}", ret);
}
