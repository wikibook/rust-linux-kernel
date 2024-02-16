fn main() {
    let vec = vec![1, 2, 3];
    for item in vec.iter() { // vec의 불변 빌림 반복자 생성
        println!("{}", item);
    }

    println!("{:?}", vec); // vec 접근 가능
}