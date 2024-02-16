use hyper::body::Buf;
use hyper::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();
    
    let client = Client::new();
    let res = client.get(url).await.unwrap();
    let body = hyper::body::aggregate(res).await.unwrap();
    
    let users: Vec<User> = serde_json::from_reader(body.reader()).unwrap();
    
    println!("users: {:#?}", users);
}