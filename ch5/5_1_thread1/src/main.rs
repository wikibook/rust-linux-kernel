use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("스레드에서 실행");
    });

    handle.join().unwrap(); // 종료 대기
}
