use std::rc::Rc;
use std::collections::HashMap;

trait Flyweight {
    fn get_name(&self) -> String;
}

struct ConcreteFlyweight {
    name: String,
}

impl Flyweight for ConcreteFlyweight {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct FlyweightFactory {
    flyweights: HashMap<String, Rc<Box<dyn Flyweight>>>,
}

impl FlyweightFactory {
    fn new() -> FlyweightFactory {
        FlyweightFactory {
            flyweights: HashMap::new(),
        }
    }

    fn get_flyweight(&mut self, name: String) -> Rc<Box<dyn Flyweight>> {
        if let Some(instance) = self.flyweights.get(&name) {
            return instance.clone();
        }

        let instance = Box::new(ConcreteFlyweight {
            name: name.clone(),
        });

        let instance = Rc::new(instance as Box<dyn Flyweight>);
        self.flyweights.insert(name.clone(), instance.clone());

        instance.clone()
    }
}

fn main() {
    let mut factory = FlyweightFactory::new();

    let flyweight1 = factory.get_flyweight(String::from("위키"));
    let flyweight2 = factory.get_flyweight(String::from("북스"));
    let flyweight3 = factory.get_flyweight(String::from("북스"));

    println!("{}", flyweight1.get_name());
    println!("{}", flyweight2.get_name());
    println!("{}", flyweight3.get_name());
}
