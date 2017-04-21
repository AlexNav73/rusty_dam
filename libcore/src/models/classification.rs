
use uuid::Uuid;

use {Entity, ToDto, FromDto, Load, LoadError};
use models::es::ClassificationDto;
use models::pg::ClassificationNamePath;
use connection::App;

pub struct Classification {
    id: Uuid,
    name_path: ClassificationNamePath,
    application: App,
}

impl Classification {
    pub fn save(&mut self) -> Result<(), LoadError> {
        let dto = self.to_dto();
        self.application
            .es()
            .index(&dto)
            .map_err(|_| LoadError::NotFound)
    }

    // TODO: Delete classification from PostgreSQL too ... 
    fn delete(mut self) -> Result<(), LoadError> {
        self.application
            .es()
            .delete::<ClassificationDto>(self.id)
            .map_err(|_| LoadError::NotFound)
    }

    pub fn new<N: Into<ClassificationNamePath>>(app: App, name_path: N) -> Self {
        Classification {
            id: Uuid::new_v4(),
            name_path: name_path.into(),
            application: app
        }
    }
}

impl Entity for Classification {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl ToDto for Classification {
    type Dto = ClassificationDto;

    fn to_dto(&self) -> ClassificationDto {
        ClassificationDto {
            id: self.id,
            name_path: self.name_path.to_string()
        }
    }
}

impl FromDto for Classification {
    type Dto = ClassificationDto;

    fn from_dto(dto: Self::Dto, app: App) -> Classification {
        Classification {
            id: dto.id,
            name_path: dto.name_path.parse().unwrap(),
            application: app,
        }
    }
}

impl Load for Classification {
    fn load(_app: App, _id: Uuid) -> Result<Self, LoadError> {
        unimplemented!()
    }
}
