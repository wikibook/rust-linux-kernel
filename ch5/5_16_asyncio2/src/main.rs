use tokio::io::{stdin, BufReader, AsyncBufReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(stdin());
    let mut lines = reader.lines();

    loop { // quit가 입력될때 까지 입력을 받음
        match lines.next_line().await.unwrap() {
            Some(input) => {
                println!("입력: {}", input);
        
                if input == "quit" {
                    break;
                }
            }
            None => {
                break;
            },
        };
    }
}