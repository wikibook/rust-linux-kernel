fn main() {
    let s: &str = "  Hello Rust ";  
    println!("{}", s.trim());  
    println!("{}", s.to_lowercase());
    println!("{}", s.to_uppercase());
}