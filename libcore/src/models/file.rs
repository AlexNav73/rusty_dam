
use uuid::Uuid;

use std::path::{Path, PathBuf};

use {Entity, ToDto, FromDto};
use models::es::FileDto;
use connection::App;

pub struct File<'a> {
    id: Uuid,
    path: PathBuf,
    application: App<'a>,
}

impl<'a> File<'a> {
    pub fn new<P: Into<PathBuf>>(app: App, path: P) -> Self {
        File {
            id: Uuid::new_v4(),
            path: path.into(),
            application: app,
        }
    }

    pub fn file_stem(&self) -> &str {
        self.path
            .as_path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    }
}

impl<'a> Entity for File<'a> {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl<'a> ToDto<'a> for File<'a> {
    type Dto = FileDto<'a>;

    fn to_dto(&self) -> Self::Dto {
        FileDto {
            id: self.id,
            full_file_path: self.path
                .to_str()
                .and_then(|s| Some(s.to_owned()))
                .unwrap(),
        }
    }
}

impl<'a, 'b> FromDto<'a> for File<'b> {
    type Dto = FileDto<'a>;

    fn from_dto(dto: Self::Dto, app: App) -> Self {
        File {
            id: dto.id,
            path: Path::new(dto.full_file_path.as_str()).to_path_buf(),
            application: app,
        }
    }
}
