use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut sum = 0;

        for i in 1..=100 {
            sum = sum + i;
        }

        tx.send(sum).unwrap();
    });

    let sum = rx.recv().unwrap();
    println!("1부터 100까지의 합: {}", sum);
}
