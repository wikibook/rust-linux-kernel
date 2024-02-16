use std::time::Duration;

fn main() {
    // 1초 123000000나노초를 설정합니다.
    let duration = Duration::new(1, 123_000_000); // 1.123초

    let sec = duration.as_secs();
    let nano = duration.subsec_nanos();

    println!("{}초 {}나노초", sec, nano);
}
