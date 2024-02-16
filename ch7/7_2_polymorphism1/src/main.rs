trait Hello {
    fn hello_msg(&self) -> String;
}

struct Student {
    name: String,
}

impl Hello for Student {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요! 선생님,")
    }
}

struct Teacher {
    name: String,
}

impl Hello for Teacher {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요. 오늘 수업은...")
    }
}

fn say_hello(say: &dyn Hello) {
    println!("{}", say.hello_msg());
}

fn main() {
    let student = Student { name: String::from("luna") };
    let teacher = Teacher { name: String::from("me") };

    say_hello(&student);
    say_hello(&teacher);
}