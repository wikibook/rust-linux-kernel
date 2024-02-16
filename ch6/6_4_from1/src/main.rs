#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

fn main() {
    let tuple = (1, 2);

    // 주어진 tuple을 바탕으로 Point객체를 생성
    let pt: Point = Point::from(tuple);
    
    println!("Point::from = {:?}", pt);
    
    // tuple을 기반으로 point를 생성합니다. 이때 Point::from이 호출됩니다.
    let pt: Point = tuple.into();

    println!("tuple.into = {:?}", pt);
}