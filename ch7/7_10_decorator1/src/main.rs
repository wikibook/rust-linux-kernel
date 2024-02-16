trait Control {
    fn draw(&self) -> String;
}

struct Button {
    name: String,
    decorators: Vec<Box<dyn Control>>,
}

impl Control for Button {
    fn draw(&self) -> String {
        let mut txt = format!("{}", self.name);

        for decorator in self.decorators.iter() {
            txt = format!("{} and {}", txt, decorator.draw());
        }

        txt
    }
}

impl Button {
    fn new(name: String) -> Button {
        Button {
            name: name,
            decorators: Vec::new(),
        }
    }

    fn add_decorator(& mut self, decorator: Box<dyn Control>) {
        self.decorators.push(decorator);
    }
}

struct Deco {
    name: String
}

impl Control for Deco {
    fn draw(&self) -> String {
        format!("{}", self.name)
    }
}

impl Deco {
    fn new(name: String) -> Box<Deco> {
        Box::new(
            Deco {
                name: name
            }
        )
    }
}

fn main() {
    let mut button = Button::new(String::from("참깨빵"));
    button.add_decorator(Deco::new(String::from("순쇠고기")));
    button.add_decorator(Deco::new(String::from("패티두장")));

    println!("{}", button.draw());
}
