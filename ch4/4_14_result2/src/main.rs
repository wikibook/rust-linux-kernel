use std::fs::File;

fn main() {
    let f = File::open("test.txt").expect("에러");
    println!("파일 열기 성공");
}