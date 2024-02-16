use std::env;

fn main() {
    // my_env라는 환경변수를 설정
    env::set_var("my_env", "my_value");

    // 환경변수 읽기
    match env::var("my_env") {
        Ok(value) => println!("my_env = {}", value),
        Err(e) => println!("my_env읽기 오류: {}", e),
    }

    // 환경변수 제거
    env::remove_var("my_env");
}
