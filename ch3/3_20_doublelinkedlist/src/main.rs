use std::cell::RefCell;
use std::rc::Rc;

// NodeType을 정의
type NodeType = Option<Rc<RefCell<Node>>>;

struct Node {
    item: i32,
    prev: NodeType,
    next: NodeType,
}

impl Node {
    fn new(item: i32) -> Self {
        Self {
            item,
            prev: None,
            next: None,
        }
    }
}

pub struct DoubleLinkedList {
    head: NodeType,
    tail: NodeType,
}

impl DoubleLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // 뒤에 삽입
    fn push_back(& mut self, item: i32) {
        let node = Rc::new(RefCell::new(Node::new(item)));

        // tail이 있다면 맨 뒤에 삽입
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(tail);
            self.tail = Some(node);
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    // 앞에 삽입
    fn push_front(&mut self, item: i32) {
        let node = Rc::new(RefCell::new(Node::new(item)));

        // head가 있다면 맨 앞에 삽입
        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(head);
            self.head = Some(node);
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    // 전체 코드를 순회
    fn print_all(&mut self) {
        let mut current = match &self.head {
            Some(n) => {
                n.clone()
            },
            None => {
                return;
            }
        };

        // 전체를 순회하면서 값을 출력
        loop {
            let t = current.clone();
            println!("item: {}", t.borrow().item);
            current = match &(t.borrow().next) {
                Some(n) => {
                    n
                },
                None => break,
            }.clone();
        }
    }
}

fn main() {
    let mut list = DoubleLinkedList::new();

    println!("뒤에 1,2,3 삽입");
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    list.print_all();
    
    println!("맨 앞에 0 추가");
    list.push_front(0);
    list.print_all();
}