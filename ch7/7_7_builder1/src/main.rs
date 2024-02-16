struct Burger {
    bun: String,
    patties: i32,
    sauce: String,
    extra: Vec<String>,
}

impl Burger {
    fn to_string(&self) -> String {
        let mut txt = format!("{} 위에 순 쇠고기 패티 {}장 {} 소스 ", 
            self.bun, self.patties, self.sauce);

        for ex in self.extra.iter() {
            txt = format!("{} {} ", txt, ex);
        }

        txt
    }
}

struct BurgerBuilder {
    bun: String,
    patties: i32,
    sauce: String,
    extra: Vec<String>,
}

impl BurgerBuilder {
    fn new() -> BurgerBuilder {
        BurgerBuilder {
            bun: String::from(""),
            patties: 0,
            sauce: String::from(""),
            extra: Vec::<String>::new()
        }
    }

    fn bun(mut self, bun: String) -> BurgerBuilder {
        self.bun = bun;
        self
    }

    fn patties(mut self, patties: i32) -> BurgerBuilder {
        self.patties = patties;
        self
    }

    fn sauce(mut self, sauce: String) -> BurgerBuilder {
        self.sauce = sauce;
        self
    }

    fn add_extra(mut self, val: String) -> BurgerBuilder {
        self.extra.push(val);
        self
    }

    fn build(self) -> Burger {
        Burger {
            bun: self.bun,
            patties: self.patties,
            sauce: self.sauce,
            extra: self.extra,
        }
    }
}

fn main() {
    let burger = BurgerBuilder::new()
        .bun(String::from("참깨빵"))
        .patties(2)
        .sauce(String::from("특별한"))
        .add_extra(String::from("양상추"))
        .build();

    println!("{}", burger.to_string());
}