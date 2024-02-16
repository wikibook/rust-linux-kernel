use std::fs::File;
use std::io::Read;
use std::os::unix::io::{FromRawFd, IntoRawFd};

fn main() {
    // test.txt를 오픈합니다.
    let f = File::open("test.txt").unwrap();

    // f의 file descriptor를 획득합니다.
    let fd = f.into_raw_fd();

    // file descriptor로부터 File객체를 생성합니다.
    let mut f = unsafe { File::from_raw_fd(fd) };
    
    // 파일 내용을 출력합니다.
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("파일 읽기 실패");
    println!("{}", contents);
}
