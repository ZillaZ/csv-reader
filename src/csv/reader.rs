use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::get_value;

use super::types::User;

pub struct Reader {
    default_password: String,
    vars: Rc<RefCell<HashMap<String, String>>>
}

impl Reader {
    pub fn new(vars: Rc<RefCell<HashMap<String, String>>>) -> Self {
        let default_password = get_value(vars.clone(), "DEFAULT_PASSWORD");
        Self {
            default_password,
            vars
        }
    }

    pub fn process_csv(&self) -> Vec<User> {
        let content = read_csv(self.vars.clone());
        content.split(",").map(|x| User::new(x, &self.default_password)).collect()
    }
}

fn read_csv(vars: Rc<RefCell<HashMap<String, String>>>) -> String {
    let csv_path = get_value(vars, "CSV_PATH");
    std::fs::read_to_string(csv_path).unwrap()
}
