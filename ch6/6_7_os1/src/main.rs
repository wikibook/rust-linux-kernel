use std::path::Path;
use std::os::unix::fs::PermissionsExt;

fn main() {
    // 사전에 touch test.txt파일을 만들어 두시면 좋습니다.
    let path = Path::new("test.txt");
    let metadata = path.metadata().unwrap();
    
    // 리눅스에서만 동작합니다.
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    println!("파일 접근 권한: {:o}", mode);
}
