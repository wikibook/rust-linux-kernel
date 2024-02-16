use std::thread;
use std::time::Duration;
use futures::executor::block_on;

async fn sleep_10sec() {
    for i in 1..10 {
        println!(".");
        thread::sleep(Duration::from_millis(1000)); // 1초간 10회 대기
    }
}

async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        println!("{} ", i);
        sum += i;
    }

    sum
}

async fn calc() -> i32 {
    let f1 = sleep_10sec();
    let f2 = calc_sum(1, 10);
    let (_, sum) = futures::join!(f1, f2);

    sum
}

fn main() {
    let future = calc();
    let sum = block_on(future);
    println!("1부터 10까지의 합: {}", sum);
}