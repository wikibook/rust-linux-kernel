macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    let sum = add!(1, 2);
    println!("1+2={}", sum);
}
