use csv::reader::Reader;
use dotenvy::dotenv;
use mysql::sql::insert_users;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub mod csv;
pub mod mysql;

pub fn get_value(vars: Rc<RefCell<HashMap<String, String>>>, value: &str) -> String {
    let borrow = vars.borrow();
    borrow.get(value).unwrap().clone()
}

fn start_env() -> HashMap<String, String> {
    dotenv().expect("Unable to start env");
    std::env::vars().collect::<HashMap<String, String>>()
}

fn main() {
    let env = Rc::new(RefCell::new(start_env()));
    let reader = Reader::new(env.clone());
    let users = reader.process_csv();
    insert_users(users, env.clone());
}
