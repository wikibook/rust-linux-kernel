struct Person {
    name: String,
    age: u32,
}

impl AsRef<str> for Person {
    // Person의 name을 str형태로 참조할 수 있습니다.
    fn as_ref(&self) -> &str {
        &self.name
    }
}

fn greet_person<P: AsRef<str>>(person: P) {
    println!("안녕! {}!", person.as_ref());
}

fn main() {
    let person = Person { name: String::from("루나"), age: 30 };

    // Person 구조체에 AsRef<str>를 구현했기 때문에, 
    // greet_person 함수는 Person을 인자로 받아 사용할 수 있습니다.
    greet_person(person);

    // 물론, String과 &str도 여전히 함수의 인자로 사용할 수 있습니다.
    let name_string = String::from("러스트");
    greet_person(name_string);
    greet_person("하이!");
}