use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut thread_vec = vec![];

    for _ in 0..100 {
        let _cnt = counter.clone(); // counter변수를 공유
        let th = thread::spawn(move || {
            let mut num = _cnt.lock().unwrap();
            *num = *num + 1;
        });
        thread_vec.push(th);
    }

    for th in thread_vec {
        th.join().unwrap();
    }

    println!("결과: {}", *counter.lock().unwrap());
}