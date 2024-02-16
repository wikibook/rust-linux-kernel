fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // i32타입을 가지는 5개 원소

    for i in 0..arr.len() {
        print!("{},", arr[i]);
    }
    println!("");
}
