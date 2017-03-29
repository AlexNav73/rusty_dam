
use uuid::Uuid;

use std::cell::RefCell;
use std::rc::Rc;

use {Entity, Document};
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
    type Dto = ClassificationDto;

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

    fn map(&self) -> ClassificationDto {
        ClassificationDto {
            id: self.id,
            full_path: match self.full_path {
                None => panic!("Classification mast have path"),
                Some(ref s) => s.to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassificationDto {
    id: Uuid,
    full_path: String,
}

impl Document<Classification> for ClassificationDto {
    fn doc_type() -> &'static str {
        "classifications"
    }

    fn map(self, conn: Rc<RefCell<Connection>>) -> Classification {
        Classification {
            id: self.id,
            full_path: Some(self.full_path),
            connection: conn,
        }
    }
}
