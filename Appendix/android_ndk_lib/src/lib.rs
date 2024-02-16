use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

// 이름 순서: Java_패키지명_액티비티명_함수명
#[no_mangle]
pub extern "system" fn Java_com_example_myapplication_MainActivity_helloRust<'local>(mut env: JNIEnv<'local>,
                                                     class: JClass<'local>,
                                                     input: JString<'local>)
                                                     -> jstring {
    let input: String =
        env.get_string(&input).expect("파라미터가 없습니다.").into();
    let output = env.new_string(format!("Hello, {}!", input))
        .expect("문자열 생성 실패");
    output.into_raw()
}