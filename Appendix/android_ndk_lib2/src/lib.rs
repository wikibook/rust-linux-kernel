use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

extern crate nalgebra as na;

use na::{DMatrix};

// 이름 순서: Java_패키지명_액티비티명_함수명
#[no_mangle]
pub extern "system" fn Java_com_example_myapplication_MainActivity_powMetrics<'local>(mut env: JNIEnv<'local>,
                                                     class: JClass<'local>,
                                                     input: JString<'local>)
                                                     -> jstring {
    // 10000 x 10000
    let mat = DMatrix::from_element(10000, 10000, 1.0);

    // 행렬 곱셈을 사용하여 제곱
    let result = &mat * &mat;

    let output = env.new_string(format!("첫번째 원소: {}!", result[(0, 0)]))
        .expect("문자열 생성 실패");
    output.into_raw()
}