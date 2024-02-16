use std::thread::sleep;
use std::time::Duration;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use signal_hook::consts::SIGINT;

fn main() -> Result<(), Error> {
    // AtomicBool을 사용하여 SIGINT 신호가 수신되었는지 여부를 확인하는 플래그를 설정합니다.
    let signal_received = Arc::new(AtomicBool::new(false));
    // AtomicBool에 대한 참조를 복제합니다. 이것은 signal_hook에 전달됩니다.
    let signal_received_clone = signal_received.clone();

    // SIGINT 신호를 수신할 때마다 signal_received_clone의 값을 true로 설정하는 핸들러를 등록합니다.
    signal_hook::flag::register(SIGINT, signal_received_clone)?;

    println!("SIGINT를 수신하거나 Ctrl+C를 입력하면 종료합니다.");

    // SIGINT 신호를 수신할 때까지 대기합니다.
    while !signal_received.load(Ordering::Relaxed) {
        sleep(Duration::from_secs(1));
    }

    println!("SIGINT수신.");

    Ok(())
}
