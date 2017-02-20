
use uuid::Uuid;
use chrono::{DateTime, UTC};

use {Entity, Document};
use file::{File, FileCollection};
use field::FieldCollection;
use classification::ClassificationCollection;
use es::SystemInfo;

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
}

impl Record {
    fn create() -> Record {
        Record {
            id: Uuid::new_v4(),
            name: None,
            fields: FieldCollection::new(),
            classifications: ClassificationCollection::new(),
            files: FileCollection::new(),
            created_on: UTC::now(),
            modified_on: UTC::now(),
            // TODO: Proper impl
            created_by: "".to_string(),
            modified_by: "".to_string(),
            is_new: true,
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

// impl From<Record> for RecordDto {
// fn from(record: Record) -> RecordDto {
// }
// }

impl Document<Record> for RecordDto {
    fn doc_type() -> &'static str {
        "record"
    }

    fn map(self) -> Record {
        Record {
            id: self.system.id,
            name: Some(self.name),
            fields: self.fields.iter().collect(),
            classifications: self.classifications.iter().collect(),
            files: self.files.iter().collect(),
            created_by: self.system.created_by.to_string(),
            created_on: DateTime::from_utc(self.system.created_on, UTC),
            modified_by: self.system.modified_by.to_string(),
            modified_on: DateTime::from_utc(self.system.modified_on, UTC),
            is_new: false,
        }
    }
}

impl Entity for Record {
    type Dto = RecordDto;

    fn id(&self) -> Uuid {
        self.id
    }

    fn map(&self) -> RecordDto {
        unimplemented!()
        // RecordDto {
        // name: self.name().to_string(),
        // fields: self.fields.iter().collect(),
        // classifications: self.classifications.iter().collect(),
        // files: self.files.iter().collect(),
        // system: SystemInfo {
        // id: self.id,
        // created_by: self.created_by,
        // created_on: self.created_on.naive_utc(),
        // modified_by: self.modified_by,
        // modified_on: self.modified_on.naive_utc(),
        // }
        // }
    }
}
