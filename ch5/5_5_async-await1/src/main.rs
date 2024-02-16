use futures::executor::block_on;

async fn hello_world() {
    println!("future안에서 실행");
}

fn main() {
    let future = hello_world();
    println!("main함수에서 실행");
    block_on(future);
    println!("future종료 이후 실행");
}