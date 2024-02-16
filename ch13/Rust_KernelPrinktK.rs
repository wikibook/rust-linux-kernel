// SPDX-License-Identifier: GPL-2.0
use kernel::prelude::*;
use kernel::c_types;

// 컴파일을 위해서는 Rust For Linux 빌드 환경이 셋팅 되어 있어야 합니다.
// 자세한 내용은 https://docs.kernel.org/rust/quick-start.html 참고해주세요.

// 커널 함수 `printk`를 위한 FFI 선언
extern “"C”" {
    fn printk(fmt: *const c_types::c_char, ...) -> c_types::c_int;
}

fn rust_printk(message: &str) {
    // Rust 문자열을 C 문자열로 변환
    let cstr = core::ffi::CString::new(message).unwrap();

    // `printk` 함수 호출을 위해 `unsafe` 블록 사용
    unsafe {
        printk(cstr.as_ptr());
    }
}

struct RustKernelPrintkModule;

impl kernel::Module for RustKernelPrintkModule {
    fn init() -> Result<Self> {
        rust_printk(“"Hello, Rust kernel module!\n”");
        Ok(RustKernelPrintkModule)
    }
}

kernel::module!(
    type: RustKernelPrintkModule,
    name: “"rust_kernel_printk”",
    author: “"Rust for Linux Contributors”",
    description: “"Rust kernel module using printk”",
    license: “"GPL”",
);
