use std::process::Command;

fn main() {
    // 쉘에서 echo를 실행합니다.
    let echo = Command::new("echo")
        .arg("echo실행") // "echo실행"이라는 인자를 추가합니다.
        .output()
        .expect("echo 실행 실패");

    // 명령어의 출력을 UTF-8 문자열로 변환합니다. 
    let ret = String::from_utf8_lossy(&echo.stdout);
    
    println!("결과: {}", ret);
}
