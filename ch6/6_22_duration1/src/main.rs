use std::time::Duration;

fn main() {
    let start = Duration::new(1, 0);
    let adder = Duration::new(1, 0);

    let sum = start + adder;
    let minus = start - adder;

    println!("Duration간 덧셈 결과: {:?}", sum);
    println!("Duration간 뺄셈 결과: {:?}", minus);
}
