fn main() {
    let number = 30; // 32비트 정수, let number: i32 = 30; 으로 선언해도 동일
    let long_number: i64 = 123456789123456789; // 64비트 정수
    let real = 10.22; // 실수
    let hangul_char = '러'; // 문자형

    println!("32비트 정수: {}", number);
    println!("64비트 정수: {}", long_number);
    println!("32비트 실수: {}", real);
    println!("문자: {}", hangul_char);
}
