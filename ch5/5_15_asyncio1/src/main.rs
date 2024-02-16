use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut file = File::open("input.txt").await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();

    println!("{}", contents);

    let mut file = File::create("output.txt").await.unwrap();
    file.write_all(contents.as_bytes()).await.unwrap();
}
