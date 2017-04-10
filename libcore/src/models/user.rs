
use uuid::Uuid;

use std::rc::Rc;
use std::cell::RefCell;

use {ToDto, FromDto, Load, LoadError};
use models::es::UserDto;
use connection::Connection;

pub struct User {
    id: Uuid,
    login: String,
    password: String,
    connection: Rc<RefCell<Connection>>,
}

impl User {
    fn new(login: String, password: String, conn: Rc<RefCell<Connection>>) -> User {
        User {
            id: Uuid::new_v4(),
            login: login,
            password: password,
            connection: conn,
        }
    }

    pub fn get(conn: Rc<RefCell<Connection>>) -> User {
        // TODO: Proper impl
        User {
            id: Uuid::new_v4(),
            login: "".to_string(),
            password: "".to_string(),
            connection: conn,
        }
    }

    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn save(&self) {
        unimplemented!()
    }
}

impl FromDto for User {
    type Dto = UserDto;

    fn from_dto(dto: Self::Dto, conn: Rc<RefCell<Connection>>) -> User {
        User {
            id: dto.id,
            login: dto.login,
            password: dto.passwd,
            connection: conn,
        }
    }
}

impl Load for User {
    fn load(_c: Rc<RefCell<Connection>>, _id: Uuid) -> Result<Self, LoadError> {
        unimplemented!()
    }
}

impl ToDto for User {
    type Dto = UserDto;

    fn to_dto(&self) -> UserDto {
        UserDto {
            id: self.id,
            login: self.login.clone(),
            passwd: self.password.clone(),
        }
    }
}
