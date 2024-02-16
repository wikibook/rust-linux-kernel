use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    for i in &list {
        print!("{}, ", i);
    }
}