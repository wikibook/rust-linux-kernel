use std::env;

fn main() {
    for (index, argument) in env::args().enumerate() {
        println!("인자 #{}: {}", index, argument);
    }
}
