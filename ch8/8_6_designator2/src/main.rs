macro_rules! create_accessors {
    ($name:ident, $type:ty, $setter:ident) => {
        fn $name(&self) -> &$type {
            &self.$name
        }

        fn $setter(&mut self, value: $type) {
            self.$name = value;
        }
    };
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    create_accessors!(name, String, set_name);
    create_accessors!(age, u32, set_age);
}

fn main() {
    let mut person = Person { name: "루나".to_string(), age: 10 };
    person.set_name("하이".to_string());
    person.set_age(8);

    println!("이름: {} 나이: {}", person.name, person.age)
}