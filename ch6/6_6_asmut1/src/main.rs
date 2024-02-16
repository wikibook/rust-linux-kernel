struct Person {
    name: String,
    age: u32,
}

impl AsMut<String> for Person {
    fn as_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

fn name_change<P: AsMut<String>>(person: &mut P, new_name: &str) {
    let name = person.as_mut();
    name.clear();
    name.push_str(new_name);
}

fn main() {
    let mut person = Person { 
        name: String::from("루나"), 
        age: 10 
    };

    println!("변경 전: {}", person.name);
    name_change(&mut person, "러스트");
    println!("변경 후: {}", person.name);
}