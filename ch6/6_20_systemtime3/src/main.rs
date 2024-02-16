use std::time::SystemTime;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let start = SystemTime::now();
    // 시간 비교를 위해 강제로 1초간 슬립합니다.
    sleep(Duration::from_secs(1));
    let end = SystemTime::now();

    if end > start {
        println!("시간 비교 가능");
    }
}