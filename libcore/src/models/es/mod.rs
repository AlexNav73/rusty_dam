
use uuid::Uuid;

use es::{SystemInfo, EsDto};

#[derive(Serialize, Deserialize)]
pub struct ClassificationDto {
    pub id: Uuid,
    pub full_path: String,
}

impl EsDto for ClassificationDto {
    fn doc_type() -> &'static str {
        "classifications"
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct FieldDto {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct FileDto {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct RecordDto {
    pub fields: Vec<FieldDto>,
    pub classifications: Vec<ClassificationDto>,
    pub files: Vec<FileDto>,
    pub system: SystemInfo,
}

impl EsDto for RecordDto {
    fn doc_type() -> &'static str {
        "records"
    }

    fn id(&self) -> Uuid {
        self.system.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: Uuid,
    pub login: String,
    pub passwd: String,
}
