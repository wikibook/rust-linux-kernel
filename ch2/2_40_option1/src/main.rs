fn print_optional(val: Option<String>) {
    match val {
        Some(val) => println!("{}", val),
        None => println!("None"),
    }
}

fn main() {
    let some_string = Some(String::from("러스트"));
    let none_string: Option<String> = None;

    print_optional(some_string);
    print_optional(none_string);
}