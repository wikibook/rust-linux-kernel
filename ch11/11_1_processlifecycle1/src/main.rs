extern crate libc;
extern crate nix;

use std::process;
use std::thread;
use std::time::Duration;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult};

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("부모 프로세스: 자식 프로세스 PID {}", child);

            // 자식 프로세스의 상태를 기다림
            match waitpid(child, None) {
                Ok(WaitStatus::Exited(pid, status)) => {
                    println!("자식 프로세스 {} 종료, 종료 코드: {}", pid, status);
                }
                Ok(WaitStatus::Signaled(pid, signal, _)) => {
                    println!("자식 프로세스 {}가 시그널 {}로 종료됨", pid, signal);
                }
                Err(err) => {
                    eprintln!("waitpid 에러: {}", err);
                    process::exit(1);
                }
                _ => {}
            }
        }
        Ok(ForkResult::Child) => {
            println!("자식 프로세스: 내 PID {}", process::id());

            // 자식 프로세스가 일정 시간 동안 실행된 후 종료
            thread::sleep(Duration::from_secs(2));
            println!("자식 프로세스 종료");
        }
        Err(err) => {
            eprintln!("fork 에러: {}", err);
            process::exit(1);
        }
    }
}
