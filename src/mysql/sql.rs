use std::{cell::RefCell, collections::HashMap, rc::Rc};
use mysql::{prelude::Queryable, *};

use crate::{csv::types::User, get_value};

pub fn insert_users(users: Vec<User>, vars: Rc<RefCell<HashMap<String, String>>>) {
    let mut conn = establish_connection(vars);
    let query = r"CALL insertUser(:email, :password);";
    conn.exec_batch(query, users.into_iter().map(|x|
        params! {
            "email" => x.email,
            "password" => x.password
        }
    )).unwrap();
}

pub fn establish_connection(vars: Rc<RefCell<HashMap<String, String>>>) -> PooledConn {
    let conn_string = get_value(vars, "CONNECTION_STRING");
    let opts = Opts::from_url(&conn_string).unwrap();
    let pool = Pool::new(opts).unwrap();
    pool.get_conn().unwrap()
}
