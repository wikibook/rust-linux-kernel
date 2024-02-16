// SPDX-License-Identifier: GPL-2.0

// 컴파일을 위해서는 Rust For Linux 빌드 환경이 셋팅 되어 있어야 합니다.
// 자세한 내용은 https://docs.kernel.org/rust/quick-start.html 참고해주세요.

use kernel::prelude::*; 
use kernel::{chrdev, file}; 

module! {
    type: RustChrdev,
    name: "rust_chrdev",
    author: "Rust for Linux Contributors",
    description: "Rust character device sample",
    license: "GPL",
} // Rust 모듈의 메타데이터 정의

struct RustFile; // RustFile 구조체 정의

#[vtable]
impl file::Operations for RustFile {
    fn open(_shared: &(), _file: &file::File) -> Result {
        Ok(())
    }
} // RustFile에 대한 파일 작업을 구현

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
} // RustChrdev 구조체 정의. 문자 장치 등록 정보 포함

impl kernel::Module for RustChrdev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust character device sample (init)\n"); // 초기화 메시지 로깅

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?; // 문자 장치 등록 생성

        // 두 개의 서브 장치(minor)를 가진 동일한 종류의 장치를 두 번 등록.
        // 여기서는 `chrdev::Registration<2>` 타입이므로 두 개의 서브 장치(minor)가 있음
        chrdev_reg.as_mut().register::<RustFile>()?;
        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n"); // 종료 메시지 로깅
    }
} // RustChrdev에 대한 소멸자 구현. 모듈 종료 시 호출됨