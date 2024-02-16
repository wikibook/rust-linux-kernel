use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let pt = Point { x: 10, y: 20 };
    let json = serde_json::to_string(&pt).unwrap();
    println!("json: {}", json);

    let pt: Point = serde_json::from_str(&json).unwrap();
    println!("point: [{}, {}]", pt.x, pt.y);
}