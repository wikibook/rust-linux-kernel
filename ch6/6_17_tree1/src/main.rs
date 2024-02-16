use std::fs;
use std::io;

// 주어진 경로에서 파일과 디렉터리 목록을 재귀적으로 출력하는 함수입니다.
// `depth`는 현재 디렉터리의 깊이를 나타내어 들여쓰기에 사용됩니다.
fn list_files_and_directories(path: &std::path::Path, depth: usize) -> io::Result<()> {
    // 주어진 경로가 디렉터리인지 확인합니다.
    if path.is_dir() {
        // 디렉터리의 모든 항목을 읽습니다.
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            // 항목의 이름을 추출합니다.
            let file_name = entry_path.file_name().and_then(|os_str| os_str.to_str()).unwrap_or("");
            // 들여쓰기와 함께 파일명 또는 디렉터리 이름을 출력합니다.
            println!("{:indent$}{}", "", file_name, indent = depth);

            // 항목이 디렉터리인 경우, 해당 디렉터리 내의 파일 및 서브디렉터리를 출력하기 위해 재귀적으로 호출합니다.
            if entry_path.is_dir() {
                list_files_and_directories(&entry_path, depth + 2)?;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    println!("{}", current_dir.display());
    list_files_and_directories(&current_dir, 0)?;

    Ok(())
}