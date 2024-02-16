use std::collections::HashMap;

fn main() {    
    let mut books: HashMap<i32, String> = HashMap::new();

    books.insert(10, String::from("Rust"));
    books.insert(20, String::from("Java"));
    books.insert(30, String::from("Python"));

    for (key, value) in &books {
        println!("Key: {:?}, Value: {:?}", key, value);
    }
}
