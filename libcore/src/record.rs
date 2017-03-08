
use uuid::Uuid;
use chrono::{DateTime, UTC};

use std::rc::Rc;
use std::cell::RefCell;

use {Entity, Document};
use file::File;
use es::SystemInfo;
use connection::Connection;

use collections::EntityCollection;
use collections::fields::FieldCollection;
use collections::files::FileCollection;
use collections::classifications::ClassificationCollection;

pub struct Record {
    id: Uuid,
    name: Option<String>,
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

impl Record {
    fn create(conn: Rc<RefCell<Connection>>) -> Record {
        Record {
            id: Uuid::new_v4(),
            name: None,
            fields: FieldCollection::new(conn.clone()),
            classifications: ClassificationCollection::new(conn.clone()),
            files: FileCollection::new(conn.clone()),
            created_on: UTC::now(),
            modified_on: UTC::now(),
            // TODO: Proper impl
            created_by: "".to_string(),
            modified_by: "".to_string(),
            is_new: true,
            connection: conn,
        }
    }

    pub fn name(&self) -> &str {
        match self.name {
            Some(ref n) => n,
            None => {
                // let file_ref: File = self.files
                // .latest()
                // .expect("File collection is empty");

                // let file: &File = file_ref.into_inner().expect("Cant load lates file");
                // file.file_stem()

                // TODO: Proper impl
                ""
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecordDto {
    name: String,
    fields: Vec<Uuid>,
    classifications: Vec<Uuid>,
    files: Vec<Uuid>,
    system: SystemInfo,
}

impl Document<Record> for RecordDto {
    fn doc_type() -> &'static str {
        "record"
    }

    fn map(self, conn: Rc<RefCell<Connection>>) -> Record {
        Record {
            id: self.system.id,
            name: Some(self.name),
            fields: FieldCollection::from_iter(self.fields.iter(), conn.clone()),
            classifications: ClassificationCollection::from_iter(self.classifications.iter(), conn.clone()),
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

impl Entity for Record {
    type Dto = RecordDto;

    fn id(&self) -> Uuid {
        self.id
    }

    fn map(&self) -> RecordDto {
        RecordDto {
            name: self.name().to_string(),
            fields: self.fields.ids().collect(),
            classifications: self.classifications.ids().collect(),
            files: self.files.ids().collect(),
            system: SystemInfo {
                id: self.id,
                created_by: self.created_by.to_string(),
                created_on: self.created_on.naive_utc(),
                modified_by: self.modified_by.to_string(),
                modified_on: self.modified_on.naive_utc(),
            }
        }
    }
}

