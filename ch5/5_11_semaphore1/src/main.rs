use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;

static counter: Mutex<i32> = Mutex::new(0);

#[tokio::main]
async fn main() {
    // 동시에 2개의 thread가 접근 가능하도록 세마포어 설정
    let semaphore = Arc::new(Semaphore::new(2));
    let mut future_vec = vec![];

    for _ in 0..100 {
        // semaphore 획득
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let future = tokio::spawn(async move {
            let mut num = counter.lock().unwrap();
            *num = *num + 1;

            drop(permit); // semaphore 해제
        });
        future_vec.push(future);
    }

    for future in future_vec {
        future.await.unwrap();
    }

    println!("결과: {}", *counter.lock().unwrap());
}