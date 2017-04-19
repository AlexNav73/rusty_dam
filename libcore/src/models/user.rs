
use uuid::Uuid;

use {ToDto, FromDto, Load, LoadError};
use models::es::UserDto;
use connection::App;

pub struct User {
    id: Uuid,
    login: String,
    password: String,
}

impl User {
    pub fn get() -> User {
        // TODO: Proper impl
        User {
            id: Uuid::new_v4(),
            login: "".to_string(),
            password: "".to_string(),
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

    fn from_dto(dto: Self::Dto, _app: App) -> User {
        User {
            id: dto.id,
            login: dto.login,
            password: dto.passwd,
        }
    }
}

impl Load for User {
    fn load(_app: App, _id: Uuid) -> Result<Self, LoadError> {
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
