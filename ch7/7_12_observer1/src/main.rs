use std::rc::Rc;

#[derive(PartialEq)]
struct Listener {}

impl Listener {
    fn on_event(&self, data: &str) {
        println!("Event 발생: {}", data);
    }
}

struct Subject {
    observers: Vec<Rc<Listener>>,
}

impl Subject {
    fn subscribe(& mut self, observer: Rc<Listener>) {
        self.observers.push(observer);
    }

    fn unsubscribe(& mut self, observer: Rc<Listener>) {
        if let Some(index) = self.observers.iter().position(|o| *o == observer) {
            self.observers.remove(index);
        }
    }

    fn notify(&self, data: &str) {
        for observer in &self.observers {
            observer.on_event(data);
        }
    }
}

fn main() {
    let mut subject = Subject {
        observers: Vec::new(),
    };

    let observer1 = Rc::new(Listener {});
    let observer2 = Rc::new(Listener {});

    subject.subscribe(observer1.clone());
    subject.subscribe(observer2.clone());

    subject.notify("새로운 이벤트");

    subject.unsubscribe(observer1.clone());

    subject.notify("새로운 이벤트");
}
