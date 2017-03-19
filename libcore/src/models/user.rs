
use uuid::Uuid;

use std::rc::Rc;
use std::cell::RefCell;

use connection::Connection;

pub struct User {
    id: Uuid,
    login: String,
    password: String,
    connection: Rc<RefCell<Connection>>
}

impl User {
    fn new(login: String, password: String, conn: Rc<RefCell<Connection>>) -> User {
        User {
            id: Uuid::new_v4(),
            login: login,
            password: password,
            connection: conn
        }
    }

    pub fn get(conn: Rc<RefCell<Connection>>) -> User {
        // TODO: Proper impl
        User {
            id: Uuid::new_v4(),
            login: "".to_string(),
            password: "".to_string(),
            connection: conn
        }
    }

    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn save(&self) {
        unimplemented!()
    }
}

