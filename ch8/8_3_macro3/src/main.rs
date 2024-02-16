macro_rules! multi_var {
    ($($var:ident: $type:ty),*) => {
        $(
            let mut $var: $type = Default::default();
        )*
    };
}

fn main() {
    multi_var!(x: u32, y: f64, z: String);
    // 컴파일러가 아래와 같이 변환합니다
    // let mut x: u32 = Default::default();
    // let mut y: f64 = Default::default();
    // let mut z: String = Default::default();
}
