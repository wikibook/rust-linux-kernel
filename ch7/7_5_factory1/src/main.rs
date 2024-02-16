trait Pizza {
    fn eat(&self);
}

enum PizzaType {
    Bulgogi,
    Hawaiian,
}

struct BulgogiPizza;
struct Hawaiianpizza;

impl Pizza for BulgogiPizza {
    fn eat(&self) {
        println!("불고기 냠냠");
    }
}

impl Pizza for Hawaiianpizza {
    fn eat(&self) {
        println!("파인애플 냠냠");
    }
}

trait PizzaFactory {
    fn create(pizza_type: PizzaType) -> Box<dyn Pizza>;
}

struct ConcretePizzaFactory;

impl PizzaFactory for ConcretePizzaFactory {
    fn create(pizza_type: PizzaType) -> Box<dyn Pizza> {
        match pizza_type {
            PizzaType::Bulgogi => Box::new(BulgogiPizza),
            PizzaType::Hawaiian => Box::new(Hawaiianpizza),
        }
    }
}

fn main() {
    let bulgogi = ConcretePizzaFactory::create(PizzaType::Bulgogi);
    let hawaiian = ConcretePizzaFactory::create(PizzaType::Hawaiian);

    bulgogi.eat();
    hawaiian.eat();
}
