
use uuid::Uuid;

use std::cell::RefCell;
use std::rc::Rc;

use {Entity, Document};
use connection::{App, Connection};

pub struct Classification {
    id: Uuid,
    connection: Rc<RefCell<Connection>>,
}

impl Entity for Classification {
    type Dto = ClassificationDto;

    fn id(&self) -> Uuid {
        self.id
    }

    fn map(&self) -> ClassificationDto {
        ClassificationDto { id: self.id }
    }

    fn create(app: &App) -> Classification {
        Classification {
            id: Uuid::new_v4(),
            connection: app.connection()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassificationDto {
    id: Uuid,
}

impl Document<Classification> for ClassificationDto {
    fn doc_type() -> &'static str {
        "classifications"
    }

    fn map(self, conn: Rc<RefCell<Connection>>) -> Classification {
        Classification {
            id: self.id,
            connection: conn,
        }
    }
}

