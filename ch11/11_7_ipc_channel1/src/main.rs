extern crate ipc_channel;
extern crate nix;

use ipc_channel::ipc::{self, IpcSender, IpcReceiver};
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult};
use std::process;

fn main() {
    let (tx, rx): (IpcSender<String>, IpcReceiver<String>) = ipc::channel().expect("IPC 채널 생성 실패");

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            let message = "안녕하세요, 자식 프로세스!";
            tx.send(message.to_string()).expect("부모 프로세스에서 IPC 채널에 메시지 전송 실패");

            waitpid(child, None).expect("waitpid 실패");
        }
        Ok(ForkResult::Child) => {
            let received_message = rx.recv().expect("자식 프로세스에서 IPC 채널로부터 메시지 수신 실패");
            println!("자식 프로세스가 받은 메시지: {}", received_message);
        }
        Err(err) => {
            eprintln!("fork 실패: {}", err);
            process::exit(1);
        }
    }
}
