fn main() {
    let mut x = Box::new(10);
    println!("x: {}", x);

    *x = 20; // x를 변경
    println!("x: {}", x);
}
