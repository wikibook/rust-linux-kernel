use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
    }

    // 9번째 인덱스를 찾기
    let d = list.iter().nth(9);
    println!("target: {:?}", d);
}