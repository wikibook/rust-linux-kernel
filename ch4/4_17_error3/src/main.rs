fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    let ret = div(1, 0);
    println!("ret: {}", ret);
}
