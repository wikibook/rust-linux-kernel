fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 2;
    v[1] = 3;
    v[2] = 4;

    println!("{}, {}, {}", v[0], v[1], v[2]);

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 1;
    }

    println!("{}, {}, {}", v[0], v[1], v[2]);
}
