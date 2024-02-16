pub mod presentation {
    pub mod view {
        pub fn render() {
            println!("mysystem::presentation::view::render");
        }
    }
}

pub mod business {
    pub mod user {
        pub fn create() {
            println!("mysystem::business::user::create");
        }
    }
}

pub mod database {
    pub mod user_dao {
        pub fn create() {
            println!("mysystem::database::user_dao::create");
        }
    }
}

#[test]
fn it_wors() {
    presentation::view::render();
    business::user::create();
    database::user_dao::create();
}