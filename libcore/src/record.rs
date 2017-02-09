
use uuid::Uuid;
use chrono::{ DateTime, UTC };

use { Lazy, Entity };
use file::*;
use field::*;
use classification::*;
use es::{ SystemInfo, EsDocument };

pub struct Record {
    id: Uuid,
    name: String,
    fields: Vec<Lazy<Field>>,
    classifications: Vec<Lazy<Classification>>,
    files: Vec<Lazy<File>>,
    created_by: String,
    created_on: DateTime<UTC>,
    modified_by: String,
    modified_on: DateTime<UTC>,
    is_new: bool
}

pub struct RecordBuilder {
    id: Uuid,
    name: Option<String>,
    classifications: Vec<Classification>,
    files: Vec<File>,
    created_by: String,
    created_on: DateTime<UTC>,
    modified_by: String,
    modified_on: DateTime<UTC>,
}

impl RecordBuilder {
    fn new<S: ToString>(creator: S) -> RecordBuilder {
        let now = UTC::now();

        RecordBuilder {
            id: Uuid::new_v4(),
            name: None,
            classifications: vec![],
            files: vec![],
            created_by: creator.to_string(),
            created_on: now.clone(),
            modified_by: creator.to_string(),
            modified_on: now
        }
    }

    #[inline]
    fn add_classification(&mut self, cls: Classification) -> &mut RecordBuilder {
        self.classifications.push(cls);
        self
    }

    #[inline]
    fn add_file(&mut self, file: File) -> &mut RecordBuilder {
        self.files.push(file);
        self
    }

    #[inline]
    fn create(mut self) -> Record {
        if self.classifications.is_empty() {
            panic!("Record must contains at least one classification");
        }

        let last_file = { 
            let last_file = self.files.last().expect("Record must contains at least one file");
            last_file.file_stem().to_string()
        };
        let classifications = { self.classifications.drain(..).map(|c| c.into()).collect() };
        let files = { self.files.drain(..).map(|f| f.into()).collect() };

        Record {
            id: self.id,
            name: last_file,
            fields: vec![],
            classifications: classifications,
            files: files,
            created_by: self.created_by,
            created_on: self.created_on,
            modified_by: self.modified_by,
            modified_on: self.modified_on,
            is_new: true
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RecordDto {
    name: String,
    fields: Vec<Uuid>,
    classifications: Vec<Uuid>,
    files: Vec<Uuid>,
    system: SystemInfo
}

impl From<RecordDto> for Record {
    fn from(dto: RecordDto) -> Record {
        Record {
            id: dto.system.id,
            name: dto.name,
            fields: dto.fields.iter().map(|&f| f.into()).collect(),
            classifications: dto.classifications.iter().map(|&c| c.into()).collect(),
            files: dto.files.iter().map(|&f| f.into()).collect(),
            created_by: dto.system.created_by.to_string(),
            created_on: DateTime::from_utc(dto.system.created_on, UTC),
            modified_by: dto.system.modified_by.to_string(),
            modified_on: DateTime::from_utc(dto.system.modified_on, UTC),
            is_new: false
        }
    }
}

impl From<Record> for RecordDto {
    fn from(record: Record) -> RecordDto {
        RecordDto {
            name: record.name,
            fields: record.fields.iter().map(|f| f.into()).collect(),
            classifications: record.classifications.iter().map(|c| c.into()).collect(),
            files: record.files.iter().map(|f| f.into()).collect(),
            system: SystemInfo {
                id: record.id,
                created_by: record.created_by,
                created_on: record.created_on.naive_utc(),
                modified_by: record.modified_by,
                modified_on: record.modified_on.naive_utc()
            }
        }
    }
}

impl EsDocument for RecordDto {
    fn entity_type() -> &'static str { "record" }
}

impl Entity for RecordDto {
    fn id(&self) -> Uuid { self.system.id }
}

