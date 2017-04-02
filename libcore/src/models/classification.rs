
use uuid::Uuid;

use std::cell::RefCell;
use std::rc::Rc;

use {Entity, ToDto, FromDto, Load, LoadError};
use es::EsDto;
use connection::{App, Connection};

pub struct Classification {
    id: Uuid,
    full_path: Option<String>,
    connection: Rc<RefCell<Connection>>,
}

impl Classification {
    // TODO: Make name_path as ClassificationPath object
    fn set_name_path(&mut self, name_path: String) {
        self.full_path = Some(name_path)
    }
}

impl Entity for Classification {
    fn create(app: &App) -> Classification {
        Classification {
            id: Uuid::new_v4(),
            full_path: None,
            connection: app.connection(),
        }
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

impl ToDto for Classification {
    type Dto = ClassificationDto;

    fn to_dto(&self) -> ClassificationDto {
        ClassificationDto {
            id: self.id,
            full_path: match self.full_path {
                None => panic!("Classification mast have path"),
                Some(ref s) => s.to_string(),
            },
        }
    }
}

impl FromDto for Classification {
    type Dto = ClassificationDto;

    fn from_dto(dto: Self::Dto, conn: Rc<RefCell<Connection>>) -> Classification {
        Classification {
            id: dto.id,
            full_path: Some(dto.full_path),
            connection: conn,
        }
    }
}

impl Load for Classification {
    fn load(_c: Rc<RefCell<Connection>>, _id: Uuid) -> Result<Self, LoadError> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassificationDto {
    id: Uuid,
    full_path: String,
}

impl EsDto for ClassificationDto {
    fn doc_type() -> &'static str {
        "classifications"
    }

    fn id(&self) -> Uuid {
        self.id
    }
}
