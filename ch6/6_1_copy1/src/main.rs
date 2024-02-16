#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn add_points(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 3, y: 4 };

    // add_point의 인자로 들어가는 a, b는 copy트레잇에 의해 복제됩니다.
    let result = add_points(a, b);

    println!("{:?}", a); // a에 접근 가능
    println!("{:?}", b); // b에 접근 가능
    println!("{:?}", result);
}