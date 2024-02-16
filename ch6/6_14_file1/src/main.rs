use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // 파일 생성
    let mut file = File::create("example.txt")?;
    file.write_all(b"Hello, Rust!")?;

    // 파일 읽기
    let mut file = File::open("example.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{}", content);

    Ok(())
}