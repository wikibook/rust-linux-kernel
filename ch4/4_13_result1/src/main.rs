use std::fs::File;

fn main() {
    let result = File::open("test.txt");
    let f = match result {
        Ok(f) => f,
        Err(err) => {
            panic!("파일 열기 실패: {:?}", err)
        },
    };

    println!("파일 열기 성공");
}