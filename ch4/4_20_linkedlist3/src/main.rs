use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
    }

    for d in list.iter_mut() {
        *d += 10;
    }

    for d in list.iter() {
        print!("{:?}, ", d);
    }
}