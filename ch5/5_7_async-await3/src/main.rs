use futures::executor::block_on;

async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        sum += i;
    }

    sum
}

async fn calc() -> i32 {
    let sum1_50 = calc_sum(1, 50).await;
    let sum51_100 = calc_sum(51, 100).await;
    let ret = sum1_50 + sum51_100;

    ret
}

fn main() {
    let future = calc();
    let sum = block_on(future);
    println!("1부터 100까지의 합: {}", sum);
}