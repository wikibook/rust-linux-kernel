use memmap::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::env;
use std::time::Duration;
use std::thread;

const FILE_PATH: &str = "shared_mmap_file.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    // 실행방법: cargo run write / cargo run read
    if args.len() != 2 {
        println!("Usage: {} [write|read]", args[0]);
        return;
    }

    match args[1].as_str() {
        "write" => write_to_mmap(),
        "read" => read_from_mmap(),
        _ => println!("Use 'write' or 'read'."), // 유효하지 않은 인수를 입력하면 메시지 출력
    }
}

fn write_to_mmap() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();

    // 작성할 메시지: Hello from mmap!
    let message = b"Hello from mmap!";
    file.set_len(message.len() as u64).unwrap();

    // 파일에 대한 mutable한 메모리 맵을 생성
    let mut mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };

    // 메모리 맵에 메시지를 씀
    mmap.copy_from_slice(message);

    println!("메시지를 작성했습니다."); // 메시지 작성 완료 알림
}

fn read_from_mmap() {
    // mmap 오픈
    let file = OpenOptions::new().read(true).open(FILE_PATH).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

    let content = String::from_utf8_lossy(&mmap);
    println!("Read from mmap: {}", content);
}