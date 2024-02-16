use std::collections::BinaryHeap;

fn main() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    heap.push(3);
    heap.push(9);
    heap.push(2);
    heap.push(5);

    while heap.is_empty() == false {
        print!("{:?}, ", heap.pop());
    }
}
