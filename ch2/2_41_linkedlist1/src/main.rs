struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn append(&mut self, elem: i32) {
        match self.next {
            Some(ref mut next) => {
                next.append(elem);
            }
            None => {
                let node = Node {
                    value:  elem,
                    next: None,
                };
                self.next = Some(Box::new(node))
            }
        }
    }

    fn list(&self) {
        print!("{},", self.value);
        match self.next {
            Some(ref next) => next.list(),
            None => {}
        }
    }
}

fn main() {
    let mut head = Node {
        value: 1,
        next: None,
    };

    for i in 2..10 {
        head.append(i);
    }

    head.list();
}