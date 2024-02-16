pub mod business;
pub mod presentation;
pub mod database;

#[test]
fn it_works() {
    presentation::view::render();
    business::user::create();
    database::user_dao::create();
}