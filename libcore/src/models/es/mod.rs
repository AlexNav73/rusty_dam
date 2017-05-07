
use uuid::Uuid;

use es::{SystemInfo, EsDto};
use models::field::FieldValue;

#[derive(Serialize, Deserialize)]
pub struct ClassificationDto<'a> {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name_path: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct FieldDto<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub value: FieldValue,
}

#[derive(Serialize, Deserialize)]
pub struct FileDto<'a> {
    pub id: Uuid,
    pub full_file_path: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct RecordDto<'a> {
    pub fields: Vec<FieldDto<'a>>,
    pub classifications: Vec<ClassificationDto<'a>>,
    pub files: Vec<FileDto<'a>>,
    pub system: SystemInfo,
}

impl<'a> EsDto<'a> for RecordDto<'a> {
    fn doc_type() -> &'static str {
        "records"
    }

    fn id(&self) -> Uuid {
        self.system.id
    }
}
