// 표준 라이브러리에서 SystemTime 모듈을 가져옵니다.
use std::time::SystemTime;

fn main() {
    // 현재 시스템 시간을 가져옵니다.
    let now = SystemTime::now();

    // 현재 시스템 시간을 디버그 형식으로 출력합니다.
    println!("{:?}", now);
}