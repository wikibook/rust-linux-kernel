use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // 파일을 읽을때 까지 대기합니다.

    println!("{}", contents);

    let mut file = File::create("output.txt").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
    // 파일을 쓸때 까지 대기합니다.
}
