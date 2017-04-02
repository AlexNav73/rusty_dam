
use uuid::Uuid;

use std::rc::Rc;
use std::cell::RefCell;

use {Entity, ToDto, FromDto, Load, LoadError};
use es::EsDto;
use connection::{App, Connection};

pub struct Field {
    id: Uuid,
    connection: Rc<RefCell<Connection>>,
}

impl Entity for Field {
    fn id(&self) -> Uuid {
        self.id
    }

    fn create(app: &App) -> Field {
        Field {
            id: Uuid::new_v4(),
            connection: app.connection(),
        }
    }
}

impl ToDto for Field {
    type Dto = FieldDto;

    fn to_dto(&self) -> FieldDto {
        FieldDto { id: self.id }
    }
}

impl FromDto for Field {
    type Dto = FieldDto;

    fn from_dto(dto: Self::Dto, conn: Rc<RefCell<Connection>>) -> Field {
        Field {
            id: dto.id,
            connection: conn,
        }
    }
}

impl Load for Field {
    fn load(_c: Rc<RefCell<Connection>>, _id: Uuid) -> Result<Self, LoadError> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct FieldDto {
    id: Uuid,
}

impl EsDto for FieldDto {
    fn doc_type() -> &'static str {
        "fields"
    }

    fn id(&self) -> Uuid {
        self.id
    }
}
