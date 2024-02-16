use autocxx::prelude::*;

// C++ 코드와 상호 작용을 위한 설정을 합니다.
include_cpp! {
    // "input.h" 헤더 파일을 포함합니다.
    #include "input.h"
    // FFI 호출이 안전하지 않음을 명시합니다.
    safety!(unsafe_ffi)
    // "input.h"의 "x3" 함수를 Rust로 노출합니다.
    generate!("x3") 
    // "input.h"의 "Test" 클래스를 Rust로 노출합니다.
    generate!("Test") 
}

fn main() {
    // x3 함수를 호출합니다.
    println!("4x3={}", ffi::x3(4));
    
    // Test 클래스의 인스턴스를 생성하고 이를 박스 안에 저장합니다.
    let mut test = ffi::Test::new().within_box();
    // Test 클래스의 inc 메서드를 두 번 호출합니다.
    test.as_mut().inc();
    test.as_mut().inc();
    
    // Test 클래스의 to_string 메서드를 호출하여 결과를 출력합니다.
    println!("{}", test.to_string().as_ref().unwrap().to_string_lossy());
}