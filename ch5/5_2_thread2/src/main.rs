use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let file = File::open("file.txt").unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let txt = line.unwrap();
            println!("{}", txt);
        }
    });

    match handle.join() {
        Ok(_) => {},
        Err(e) => {
            println!("스레드 내부에서 오류가 발생했습니다. {:?}", e);
        }
    };
}