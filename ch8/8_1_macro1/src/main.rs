macro_rules! say_hello {
    ($name:expr) => {
        println!("안녕! {}!", $name);
    };
}

fn main() {
    say_hello!("러스트");
}