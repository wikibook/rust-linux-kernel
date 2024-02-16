// miette라이브러리를 사용하여 빌드 환경을 구성합니다.
fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    
    // autocxx_build의 Builder를 사용하여 "src/main.rs" 파일에 대한 빌더 인스턴스를 생성합니다.
    let mut b = autocxx_build::Builder::new("src/main.rs", [&path]).build()?;
    
    // c++14버전을 사용합니다.
    b.flag_if_supported("-std=c++14").compile("autocxx-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.h");
    
    Ok(())
}