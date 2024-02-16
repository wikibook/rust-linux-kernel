fn main() {
    let mut x = 5;
    let mut inc = || {
        x += 1;
    };
    inc();
    println!("변수 x: {}", x);
}