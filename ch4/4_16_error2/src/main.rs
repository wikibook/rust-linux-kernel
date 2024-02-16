use std::io;
use std::io::Read;
use std::fs::File;

fn read_from_file(path: String) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let ret = read_from_file(String::from("test.txt")).expect("파일 읽기 중 오류가 발생했습니다.");
    println!("test.txt: {}", ret);
}