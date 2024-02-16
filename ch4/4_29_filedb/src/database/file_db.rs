use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Error;

use crate::business::user::UserManager;
use crate::business::user::User;

pub fn save(file_name: String, user_vec: Vec<&User>) -> Result<(), Error> {
    let mut buffer = File::create(file_name).expect("파일을 열 수 없습니다.");
    for u in user_vec.iter() {
        let f = format!("{} {} {}\n", u.id, u.age, u.name);
        buffer.write(f.as_str().as_bytes())?;
    }

    Ok(())
}

pub fn load(file_name: String) -> Vec<User> {
    let mut user_vec : Vec<User> = Vec::new();
    let txt = fs::read_to_string(file_name).expect("파일을 읽을 수 없습니다.");

    for ln in txt.split("\n") {
        if ln.len() == 0 {
            break;
        }

        let tok: Vec<&str> = ln.split(" ").collect();
        user_vec.push(User {
            id: tok[0].parse::<i32>().unwrap(),
            age: tok[1].parse::<i32>().unwrap(),
            name: tok[2].to_string(),
        });
    }

    user_vec
}