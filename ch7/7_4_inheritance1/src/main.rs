trait Pointable {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

struct Point {
    x: i32,
    y: i32,
}

impl Pointable for Point {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

struct ColorPoint {
    color: String,
    point: Point,
}

impl ColorPoint {    
    fn new(color: String, x: i32, y: i32) -> ColorPoint {
        ColorPoint {
            color: color,
            point: Point {
                x: x,
                y: y
            }
        }
    }

    fn color(&self) -> &String {
        &self.color
    }
}

impl Pointable for ColorPoint {
    fn x(&self) -> i32 {
        self.point.x
    }

    fn y(&self) -> i32 {
        self.point.y
    }
}

fn print_pointable(pointable: &dyn Pointable) {
    println!("x: {} y: {}", pointable.x(), pointable.y());
}

fn main() {
    let pt = ColorPoint::new(String::from("red"), 1, 2);
    print_pointable(&pt);
}