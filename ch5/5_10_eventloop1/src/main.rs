use std::io;

fn main() {
    println!("아무 내용이나 입력하세요. quit를 입력하면 종료됩니다.");

    loop { // 무한 반복하여 이벤트를 처리
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        
        let input = buf.trim();
        if input == "quit" {
            break;
        }

        println!("입력: {}", input);
    }
}