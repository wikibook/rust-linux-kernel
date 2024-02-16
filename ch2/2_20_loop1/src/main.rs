use std::io;

fn main() {
    loop {
        println!("숫자를 입력해주세요. 0을 입력하면 종료합니다");
        let mut read = String::new();
        io::stdin().read_line(&mut read).unwrap();
        let val: i32 = read.trim().parse().unwrap();

        if val == 0 {
            break;
        }

        println!("입력={}", val);
    }
}