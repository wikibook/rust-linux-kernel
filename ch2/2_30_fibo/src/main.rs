use std::io;

fn fibo(n: i32) -> i32 {
    let mut next = 0;
    let mut t1 = 1; // 처음은 1, 1로 시작
    let mut t2 = 1;
    let mut counter = 2;

    print!("1, 1, ");
    while counter < n {
        next = t1 + t2;
        t1 = t2;
        t2 = next;
        print!("{}, ", next);

        counter += 1;
    }
    println!("");

    next // 세미콜론 주의
}

fn main() {
    println!("n번째 수를 입력해주세요.");
    
    let mut n = String::new();
    io::stdin().read_line(&mut n);
    let n: i32 = n.trim().parse().unwrap();
    println!("입력 수: {}", n);

    let ret = fibo(n);
    println!("결과: {}", ret);
}


#[test]
fn fibo_test() {
    assert_eq!(fibo(6), 8); // 6번째 수열은 8이여야 함
    assert_eq!(fibo(7), 13); // 7번째 수열은 13이여야 함
}