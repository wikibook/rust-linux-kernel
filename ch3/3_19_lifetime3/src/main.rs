static GLOBAL_CONST: i32 = 10; // 프로그램이 종료될때까지 메모리에서 해제되지 않음

fn main() {
    let x: &'static str = "Hello Rust!";
         // x는 프로그램이 종료될때까지 메모리에서 해제되지 않음
    println!("x: {}", x);
    println!("GLOBAL_CONST: {}", GLOBAL_CONST);
}