pub struct Student {
    id: i32, // private 필드
    pub name: String, // public 필드
    pub email: String, // public 필드
}

impl Student {
    // public 생성자
    pub fn new(id: i32, name: String, email: String) -> Student {
        Student { id, name, email }
    }

    // public 메서드
    pub fn get_name(&self) -> &String {
        &self.name
    }

    // private 메서드
    fn set_name(& mut self, name: String) {
        self.name = name.clone();
    }
}

fn main() {
    let student = Student::new(1, String::from("luna"), String::from("luna@email.me"));
    println!("이름: {}", student.get_name());
}
