use std::collections::HashMap;

fn main() {    
    let mut books: HashMap<i32, String> = HashMap::new();

    books.insert(10, String::from("Rust"));
    books.insert(20, String::from("Java"));
    books.insert(30, String::from("Python"));

    let rust = books.get(&10);
    println!("key 10은 {:?}", rust);
}
