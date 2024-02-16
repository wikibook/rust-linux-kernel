trait Control {
    fn draw(&self) -> String;
}

struct Button {
    name: String,
}

impl Control for Button {
    fn draw(&self) -> String {
        format!("Button - {}", self.name)
    }
}

impl Button {
    fn new(name: String) -> Box<Button> {
        Box::new(Button {
            name: name
        })
    }
}

struct Panel {
    name: String,
    controls: Vec<Box<dyn Control>>,
}

impl Control for Panel {
    fn draw(&self) -> String {
        let mut txt = format!("Panel - {}", self.name);
        for control in self.controls.iter() {
            txt = format!("{}\n\t{}", txt, control.draw());
        }
        txt
    }
}

impl Panel {
    fn new(name: String) -> Box<Panel> {
        Box::new(Panel {
            name: name,
            controls: Vec::new()
        })
    }

    fn add_control(& mut self, control: Box<dyn Control>) {
        self.controls.push(control);
    }
}

fn main() {
    let mut root = Panel::new(String::from("root"));
    root.add_control(Button::new( String::from("button #1")));

    let mut panel = Panel::new(String::from("panel #1"));
    panel.add_control(Button::new( String::from("button #2")));
    root.add_control(panel);

    println!("{}", root.draw());
}
