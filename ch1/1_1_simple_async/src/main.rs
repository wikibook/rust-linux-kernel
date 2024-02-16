use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // 아무것도 출력되지 않습니다.
    block_on(future); // `future` 실행되면 "hello, world!"가 출력됩니다.
}
