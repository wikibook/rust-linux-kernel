use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1); // tx1복제

    // 1부터 50까지의 합
    thread::spawn(move || {
        let mut sum = 0;

        for i in 1..=50 {
            sum = sum + i;
        }

        tx1.send(sum).unwrap();
    });

    // 51부터 100까지의 합
    thread::spawn(move || {
        let mut sum = 0;

        for i in 51..=100 {
            sum = sum + i;
        }

        tx2.send(sum).unwrap();
    });

    let mut sum = 0;
    
    for val in rx {
        println!("수신: {}", val);
        sum = sum + val;
    } 

    println!("1부터 100까지의 합: {}", sum);
}
