extern crate nix;
extern crate libc;

use libc::sched_param;
use libc::{sched_get_priority_max, sched_get_priority_min, sched_setscheduler, sched_getscheduler};
use nix::sys::wait::wait;
use nix::unistd::{fork, getpid, ForkResult};
use std::process;

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            let max_priority = unsafe { sched_get_priority_max(libc::SCHED_RR) };
            let min_priority = unsafe { sched_get_priority_min(libc::SCHED_RR) };
            let priority = (max_priority + min_priority) / 2;

            let mut param = sched_param { sched_priority: priority };
            let result = unsafe { sched_setscheduler(child.into(), libc::SCHED_RR, &param) };
        }
        Ok(ForkResult::Child) => {
            let pid = getpid();
            std::thread::sleep(std::time::Duration::from_secs(1));
            let scheduling_policy = unsafe { sched_getscheduler(pid.into()) };

            println!("자식 프로세스 PID: {:?}", pid);
            println!("자식 프로세스 스케줄링 정책: {}", scheduling_policy);

            std::thread::sleep(std::time::Duration::from_secs(5));
            println!("자식 프로세스 종료");
        }
        Err(err) => {
            eprintln!("fork 실패: {}", err);
            process::exit(1);
        }
    }
}
