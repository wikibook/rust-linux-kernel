macro_rules! add {
    ($x:expr, $y:expr) => {
        $x + $y
    };
}

macro_rules! multiply {
    ($x:expr, $y:expr) => {
        $x * $y
    };
}

macro_rules! compute {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        multiply!(add!($a, $b), add!($c, $d))
    };
}

fn main() {
    let result = compute!(1, 2, 1, 2); 
    // (1 + 2) * (1 + 2) = 3 * 3 = 9
    println!("(1+2)x(1+2)={}", result);
}
