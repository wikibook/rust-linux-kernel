extern crate nix;
extern crate libc;

use nix::sys::wait::waitpid;
use nix::unistd::{close, fork, pipe, read, write, ForkResult};
use std::process;
use std::str;

fn main() {
    let (read_fd, write_fd) = pipe().expect("pipe 실패");

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            close(read_fd).expect("부모 프로세스에서 읽기 디스크립터 닫기 실패");

            let message = "안녕하세요, 자식 프로세스!";
            write(write_fd, message.as_bytes()).expect("부모 프로세스에서 파이프 쓰기 실패");

            close(write_fd).expect("부모 프로세스에서 쓰기 디스크립터 닫기 실패");

            waitpid(child, None).expect("waitpid 실패");
        }
        Ok(ForkResult::Child) => {
            close(write_fd).expect("자식 프로세스에서 쓰기 디스크립터 닫기 실패");

            let mut buf = [0u8; 1024];
            let nbytes = read(read_fd, &mut buf).expect("자식 프로세스에서 파이프 읽기 실패");

            close(read_fd).expect("자식 프로세스에서 읽기 디스크립터 닫기 실패");

            let received_message = str::from_utf8(&buf[..nbytes]).expect("UTF-8 문자열 변환 실패");
            println!("자식 프로세스가 받은 메시지: {}", received_message);
        }
Err(err) => {
            eprintln!("fork 실패: {}", err);
            process::exit(1);
        }
    }
}
