
use uuid::Uuid;

use std::cell::RefCell;
use std::rc::Rc;

use {Entity, Document};
use connection::Connection;

pub struct Classification {
    id: Uuid,
    connection: Rc<RefCell<Connection>>
}

#[derive(Serialize, Deserialize)]
pub struct ClassificationDto {
    id: Uuid
}

impl Document<Classification> for ClassificationDto {
    fn doc_type() -> &'static str {
        "classification"
    }

    fn map(self, conn: Rc<RefCell<Connection>>) -> Classification {
        Classification {
            id: self.id,
            connection: conn
        }
    }
}

impl Entity for Classification {
    type Dto = ClassificationDto;

    fn id(&self) -> Uuid {
        self.id
    }

    fn map(&self) -> ClassificationDto {
        ClassificationDto {
            id: self.id
        }
    }
}

