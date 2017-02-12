
use uuid::Uuid;
use chrono::{DateTime, UTC};

use {Lazy, Entity};
use file::{File, FileCollection};
use field::FieldCollection;
use classification::ClassificationCollection;
use es::{SystemInfo, EsDocument};

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
                let file: &File = self.files.latest().expect("File collection is empty").into();
                file.file_stem()
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RecordDto {
    name: String,
    fields: Vec<Uuid>,
    classifications: Vec<Uuid>,
    files: Vec<Uuid>,
    system: SystemInfo,
}

impl From<RecordDto> for Record {
    fn from(dto: RecordDto) -> Record {
        Record {
            id: dto.system.id,
            name: Some(dto.name),
            fields: dto.fields.iter().collect(),
            classifications: dto.classifications.iter().collect(),
            files: dto.files.iter().collect(),
            created_by: dto.system.created_by.to_string(),
            created_on: DateTime::from_utc(dto.system.created_on, UTC),
            modified_by: dto.system.modified_by.to_string(),
            modified_on: DateTime::from_utc(dto.system.modified_on, UTC),
            is_new: false,
        }
    }
}

impl From<Record> for RecordDto {
    fn from(record: Record) -> RecordDto {
        RecordDto {
            name: record.name().to_string(),
            fields: record.fields.iter().map(|f| f.into()).collect(),
            classifications: record.classifications.iter().map(|c| c.into()).collect(),
            files: record.files.iter().map(|f| f.into()).collect(),
            system: SystemInfo {
                id: record.id,
                created_by: record.created_by,
                created_on: record.created_on.naive_utc(),
                modified_by: record.modified_by,
                modified_on: record.modified_on.naive_utc(),
            },
        }
    }
}

impl EsDocument for RecordDto {
    fn entity_type() -> &'static str {
        "record"
    }
}

impl Entity for RecordDto {
    fn id(&self) -> Uuid {
        self.system.id
    }
}
