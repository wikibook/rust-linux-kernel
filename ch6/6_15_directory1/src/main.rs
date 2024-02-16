use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // 디렉터리 생성
    fs::create_dir("example_directory")?;
    println!("example_directory 생성됨");

    // 현재 실행 디렉터리의 모든 내용 출력
    let entries = fs::read_dir(".")?;
    println!("현재 실행 디렉터리 내용:");
    for entry in entries {
        let entry = entry?;
        println!("{:?}", entry.path());
    }

    // 디렉터리 삭제
    fs::remove_dir("example_directory")?;
    println!("example_directory 삭제됨");

    Ok(())
}