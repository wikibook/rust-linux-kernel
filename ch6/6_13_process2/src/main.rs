use std::process;

fn main() {
    let pid = process::id(); // 프로세스의 pid를 획득
    println!("Process ID: {}", pid);
}
