use std::thread;
use std::sync::Mutex;

static counter: Mutex<i32> = Mutex::new(0); // counter를 전역변수로 정의

fn inc_counter() {
    let mut num = counter.lock().unwrap();
    *num = *num + 1;
} // inc_counter를 벗어나는 순간 counter는 unlock됩니다.

fn main() {
    let mut thread_vec = vec![];

    for _ in 0..100 {
        let th = thread::spawn(inc_counter);
        thread_vec.push(th);
    }

    for th in thread_vec {
        th.join().unwrap();
    }

    println!("결과: {}", *counter.lock().unwrap());
}