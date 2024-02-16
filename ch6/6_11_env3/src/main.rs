use std::env;

fn main() {
    match env::current_dir() {
        Ok(path) => println!("현재 경로: {:?}", path),
        Err(e) => println!("현재 경로 획득 실패: {}", e),
    }
    
    match env::temp_dir().to_str() {
        Some(path) => println!("임시 경로: {}", path),
        None => println!("임시 경로 확인 불가")
    }
}
