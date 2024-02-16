use std::path::{Path, PathBuf};

fn main() {
    // Path 생성
    let path = Path::new("/tmp/test.txt");

    // 경로의 파일명 추출
    if let Some(filename) = path.file_name() {
        println!("파일명: {:?}", filename);
    }

    // 경로의 확장자 추출
    if let Some(extension) = path.extension() {
        println!("확장자: {:?}", extension);
    }

    // 경로 조작하기 위한 PathBuf 생성
    let mut path_buf = PathBuf::from("/tmp/foo");
    
    // 경로에 파일명 추가
    path_buf.push("example.txt");
    println!("전체 경로: {:?}", path_buf);  
}
