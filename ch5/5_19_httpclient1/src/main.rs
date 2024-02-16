use hyper::{body::HttpBody as _, Client};
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse().unwrap();
    let mut resp = client.get(uri).await.unwrap();
    println!("Response: {}", resp.status());
    
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk.unwrap()).await.unwrap();
    }
}