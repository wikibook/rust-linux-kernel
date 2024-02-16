struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Book객체 해제: {}", self.title);
    }
}

fn main() {
    {
        let book = Book { title: String::from("러스트") };
    } // book의 Drop트레잇이 자동으로 호출됩니다.
}
