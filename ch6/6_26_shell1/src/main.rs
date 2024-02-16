use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop { // 무한 루프
        print!("RustShell$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        // 사용자가 'exit'를 입력하면 루프를 종료합니다.
        if input == "exit" {
            break;
        }

        // 사용자 입력을 공백을 기준으로 나누어 parts 벡터에 저장합니다.
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        // parts의 첫 번째 요소(명령어)를 command 변수에 저장합니다.
        let command = parts[0];
        // 나머지 부분은 args에 저장합니다.
        let args = &parts[1..];

        // 저장된 명령어와 인자들을 사용하여 외부 명령어를 실행합니다.
        let output = Command::new(command)
            .args(args)
            .output()
            .expect("Failed to execute command");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
