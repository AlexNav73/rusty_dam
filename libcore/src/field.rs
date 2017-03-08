
use uuid::Uuid;

use std::rc::Rc;
use std::cell::RefCell;

use {Entity, Document};
use connection::Connection;

pub struct Field {
    id: Uuid,
    connection: Rc<RefCell<Connection>>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldDto {
    id: Uuid,
}

impl Document<Field> for FieldDto {
    fn doc_type() -> &'static str {
        "field"
    }

    fn map(self, conn: Rc<RefCell<Connection>>) -> Field {
        Field {
            id: self.id,
            connection: conn,
        }
    }
}

impl Entity for Field {
    type Dto = FieldDto;

    fn id(&self) -> Uuid {
        self.id
    }

    fn map(&self) -> FieldDto {
        FieldDto { id: self.id }
    }
}
