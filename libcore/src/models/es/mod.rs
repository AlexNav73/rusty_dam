
use uuid::Uuid;

use es::{SystemInfo, EsDto, EsDocument};
use models::field::FieldValue;

#[derive(Serialize, Deserialize)]
pub struct ClassificationDto {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct FieldDto {
    pub id: Uuid,
    pub name: String,
    pub value: FieldValue,
}

#[derive(Serialize, Deserialize)]
pub struct FileDto {
    pub id: Uuid,
    pub full_file_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct RecordDto {
    pub fields: Vec<FieldDto>,
    pub classifications: Vec<ClassificationDto>,
    pub files: Vec<FileDto>,
    pub system: SystemInfo,
}

impl EsDocument for RecordDto {
    fn doc_type() -> &'static str {
        "records"
    }
}

impl EsDto for RecordDto {
    fn id(&self) -> Uuid {
        self.system.id
    }
}
