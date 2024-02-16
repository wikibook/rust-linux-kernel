enum Message {
    Quit,
    List(i32),
    Put(String),
    Get(i32),
}

impl Message {
    fn execute(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::List(val) => println!("List: {}", val),
            Message::Put(val) => println!("Put: {}", val),
            Message::Get(val) => println!("Get: {}", val),
        }
    }
}

fn main() {
    let m = Message::Put(String::from("/root/"));
    m.execute();

    let m = Message::List(33);
    m.execute();
}
