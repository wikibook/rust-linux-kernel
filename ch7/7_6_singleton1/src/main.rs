#[macro_use]
extern crate lazy_static;

struct MySingleton {
    name: String
}

impl MySingleton {
    fn new(name: String) -> MySingleton {
        MySingleton {
            name: name
        }
    }

    // methods of the singleton struct
    fn call(&self) {
        println!("my name is {}", self.name);
    }
}

lazy_static! {
    static ref INSTANCE: MySingleton = {
        MySingleton::new(String::from("luna"))
    };
}

fn main() {
    INSTANCE.call();
}