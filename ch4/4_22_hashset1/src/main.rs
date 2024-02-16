use std::collections::HashSet;

fn main() {
    let mut book : HashSet<String> = HashSet::new();
    
    book.insert(String::from("Rust"));
    book.insert(String::from("Rust"));
    book.insert(String::from("Rust"));
    book.insert(String::from("Java"));
    
    for data in &book {
        println!("{:?}", data);
    }

if book.contains("Python") == false {
    println!("Python이 없습니다.")
}
}