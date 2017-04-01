
use uuid::Uuid;
use chrono::{DateTime, UTC};

use std::rc::Rc;
use std::cell::RefCell;

use {Entity, ToDto, FromDto};
//use models::file::File;
use es::{SystemInfo, EsDto};
use connection::{App, Connection};

use models::collections::EntityCollection;
use models::collections::fields::FieldCollection;
use models::collections::files::FileCollection;
use models::collections::classifications::ClassificationCollection;

pub struct Record {
    id: Uuid,
    fields: FieldCollection,
    classifications: ClassificationCollection,
    files: FileCollection,
    created_by: String,
    created_on: DateTime<UTC>,
    modified_by: String,
    modified_on: DateTime<UTC>,
    is_new: bool,
    connection: Rc<RefCell<Connection>>,
}

impl Entity for Record {
    fn id(&self) -> Uuid {
        self.id
    }

    fn create(app: &App) -> Record {
        let conn = app.connection();

        Record {
            id: Uuid::new_v4(),
            fields: FieldCollection::new(conn.clone()),
            classifications: ClassificationCollection::new(conn.clone()),
            files: FileCollection::new(conn.clone()),
            created_on: UTC::now(),
            modified_on: UTC::now(),
            created_by: app.user().login().to_string(),
            modified_by: app.user().login().to_string(),
            is_new: true,
            connection: conn,
        }
    }
}

impl ToDto for Record {
    type Dto = RecordDto;

    fn to_dto(&self) -> RecordDto {
        RecordDto {
            fields: self.fields.ids().collect(),
            classifications: self.classifications.ids().collect(),
            files: self.files.ids().collect(),
            system: SystemInfo {
                id: self.id,
                created_by: self.created_by.to_string(),
                created_on: self.created_on.naive_utc(),
                modified_by: self.modified_by.to_string(),
                modified_on: self.modified_on.naive_utc(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecordDto {
    fields: Vec<Uuid>,
    classifications: Vec<Uuid>,
    files: Vec<Uuid>,
    system: SystemInfo,
}

impl EsDto for RecordDto {
    fn doc_type() -> &'static str {
        "records"
    }

    fn id(&self) -> Uuid {
        self.system.id
    }
}

impl FromDto for RecordDto {
    type Item = Record;

    fn from_dto(self, conn: Rc<RefCell<Connection>>) -> Record {
        Record {
            id: self.system.id,
            fields: FieldCollection::from_iter(self.fields.iter(), conn.clone()),
            classifications: ClassificationCollection::from_iter(self.classifications.iter(),
                                                                 conn.clone()),
            files: FileCollection::from_iter(self.files.iter(), conn.clone()),
            created_by: self.system.created_by.to_string(),
            created_on: DateTime::from_utc(self.system.created_on, UTC),
            modified_by: self.system.modified_by.to_string(),
            modified_on: DateTime::from_utc(self.system.modified_on, UTC),
            is_new: false,
            connection: conn,
        }
    }
}
