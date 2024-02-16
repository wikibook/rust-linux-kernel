macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("함수: {:?}()", stringify!($func_name));
        }
    };
}

// ident_func이라는 함수를 컴파일 시점에 생성
create_function!(ident_func);

fn main() {
    ident_func();
}