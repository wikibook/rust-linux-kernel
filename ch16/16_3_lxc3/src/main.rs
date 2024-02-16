extern crate lxc;

use std::os::unix::io::AsRawFd;
use lxc_sys::lxc_groups_t;

fn main() -> std::io::Result<()> {
    // playground 컨테이너를 사용합니다.
    let c =
        lxc::Container::new("playground", None).expect("Failed to setup lxc_container struct");

    // lxc설정: 전부 기본값으로 설정합니다.
    let mut options = lxc::attach::Options {
        attach_flags: 0,
        env_policy: 0,
        extra_env_vars: std::ptr::null_mut(),
        gid: 0,
        uid: 0,
        extra_keep_env: std::ptr::null_mut(),
        initial_cwd: std::ptr::null_mut(),
        log_fd: std::io::stdout().as_raw_fd(),
        stdout_fd: std::io::stdout().as_raw_fd(),
        stderr_fd: std::io::stderr().as_raw_fd(),
        stdin_fd: std::io::stdin().as_raw_fd(),
        namespaces: -1,
        personality: -1,
        lsm_label: std::ptr::null_mut(),
        groups: lxc_groups_t {
            list: std::ptr::null_mut(),
            size: 0,
        },
    };

    // 쉘 구동
    let prog = "/bin/bash";
    let args = [];
    let r = c.attach_run_wait(&mut options, prog, &args);
    match r {
        Err(e) => println!("Error: {}", e),
        Ok(s) => println!("Ok, waitpid() status={}", s),
    }

    Ok(())
}