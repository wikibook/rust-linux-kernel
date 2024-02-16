// SPDX-License-Identifier: GPL-2.0

// 컴파일을 위해서는 Rust For Linux 빌드 환경이 셋팅 되어 있어야 합니다.
// 자세한 내용은 https://docs.kernel.org/rust/quick-start.html 참고해주세요.

use kernel::prelude::*;
use alloc::alloc::*;

// 모듈 메타데이터 선언
module! {
    type: RustMemoryAlloc,
    name: "rust_memory_alloc",
    author: "Rust for Linux Contributors",
    description: "Rust memory allocation sample",
    license: "GPL",
}

struct RustMemoryAlloc;

impl kernel::Module for RustMemoryAlloc {
    // 모듈이 초기화될 때 호출되는 함수
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust memory allocation sample (init)\n");

        pr_info!("Global allocators\n");

        // 메모리 직접 접근을 위해 unsafe 사용
        unsafe {
            // `u16` 타입에 대한 메모리 레이아웃 생성
            let layout = Layout::new::<u16>();
            // 레이아웃에 따라 메모리 할당
            let ptr = alloc(layout);

            // 할당된 메모리에 대한 참조 및 값 할당
            *(ptr as *mut u16) = 42;
            // 메모리 할당 및 할당이 예상대로 작동했는지 확인
            assert_eq!(*(ptr as *mut u16), 42);

            // 메모리 해제
            dealloc(ptr, layout);
        }

        // 메모리 직접 접근을 위해 unsafe 사용
        unsafe {
            let layout = Layout::new::<u16>();
            let ptr = alloc_zeroed(layout);

            // 메모리가 제로로 초기화되었는지 확인
            assert_eq!(*(ptr as *mut u16), 0);

            // 메모리 해제
            dealloc(ptr, layout);
        }

        // 힙에 메모리 할당
        pr_info!("Heap allocation\n");
        let val: u32 = 5;
        
        let boxed = Box::try_new(val)?;
        pr_info!("boxed : {}\n",boxed);

        let boxed = Box::try_new(6)?;
        let val: u32 = *boxed;
        pr_info!("val : {}\n",val);

        pr_info!("Vector allocation\n");
        let mut v: Vec<i32> = Vec::new();
        
        v.try_push(1)?;
        v.try_push(2)?;
        v.try_push(3)?;
        pr_info!("v[0]:{}, v[1]:{}. v[2]:{}\n",v[0],v[1],v[2]);

        // 초기화 성공
        Ok(RustMemoryAlloc)
    }
}

impl Drop for RustMemoryAlloc {
    // 모듈이 언로드될 때 호출되는 함수
    fn drop(&mut self) {
        pr_info!("Rust memory allocation sample (exit)\n");
    }
}