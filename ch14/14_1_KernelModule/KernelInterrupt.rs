// SPDX-License-Identifier: GPL-2.0

// 컴파일을 위해서는 Rust For Linux 빌드 환경이 셋팅 되어 있어야 합니다.
// 자세한 내용은 https://docs.kernel.org/rust/quick-start.html 참고해주세요.

use kernel::prelude::*;
use kernel::sync::{CondVar, Mutex};
use kernel::task::Task;

module! {
    type: KernelThread,
    name: "kernel_thread_creation",
    author: "Rust for Linux Contributors",
    description: "Rust kernel therad sample",
    license: "GPL",
}

struct KernelThread;  // KernelThread 구조체 정의

// 정적 동기화 변수를 초기화한다.
kernel::init_static_sync! {
    static COUNT: Mutex<u32> = 0;  // COUNT 뮤텍스로 래핑된 정수 변수
    static COUNT_IS_ZERO: CondVar;  // 조건 변수
}

// 스레드 함수 정의
fn threadfn() {
    // 현재 실행 중인 스레드의 정보를 출력한다.
    pr_info!("Running from thread {}\n", Task::current().pid());
    let mut guard = COUNT.lock();  // COUNT 뮤텍스를 잠근다.
    *guard -= 1;
    if *guard == 0 {
        COUNT_IS_ZERO.notify_all();  // 모든 대기 중인 스레드에 알린다.
    }
}

impl kernel::Module for KernelThread {
    // 모듈 초기화 함수
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust kernel thread creation sample (init)\n");  // 초기화 시작 메시지

        *COUNT.lock() = 10;  // COUNT 변수를 10으로 설정한다.

        pr_info!("10 kernel thread creation\n");  // 10개의 커널 스레드 생성 메시지
        for i in 0..10 {
            Task::spawn(fmt!("test{i}"), threadfn).unwrap();  // 10개의 스레드를 생성한다.
        }

        Ok(KernelThread)
    }
}

impl Drop for KernelThread {
    // KernelThread 구조체가 소멸될 때 실행되는 함수
    fn drop(&mut self) {
        let mut guard = COUNT.lock();  // COUNT 뮤텍스를 잠근다.
        while *guard != 0 {
                COUNT_IS_ZERO.wait(&mut guard);  // COUNT 값이 0이 될 때까지 대기한다.
        }

        pr_info!("Rust thread creation sample (exit)\n");  // 모듈 종료 메시지
    }
}