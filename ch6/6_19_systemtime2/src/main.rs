use std::thread::sleep;
use std::time::SystemTime;
use std::time::Duration;

fn main() {
    let tm = SystemTime::now();

    println!("1초 대기");
    sleep(Duration::from_secs(1));

    match tm.elapsed() { // 시간 측정
        Ok(elapsed) => {
            println!("대기 시간: {}.{}초", elapsed.as_secs(), elapsed.subsec_millis());
        }
        Err(e) => {
            println!("오류 발생: {:?}", e);
        }
    }
}
