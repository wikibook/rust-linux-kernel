fn main() {
    let mut v: Vec<i32> = vec![];

    for i in 1..10 {
        v.push(i);
    }

    for d in v {
        print!("{},", d);
    }
}
