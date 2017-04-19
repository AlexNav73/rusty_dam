
use uuid::Uuid;

use {Entity, ToDto, FromDto, Load, LoadError};
use models::es::ClassificationDto;
use connection::App;

pub struct Classification {
    id: Uuid,
    full_path: Option<String>,
    application: App
}

impl Classification {
    pub fn save(&mut self) -> Result<(), LoadError> {
        let dto = self.to_dto();
        self.application
            .es()
            .index(&dto)
            .map_err(|_| LoadError::NotFound)
    }

    fn delete(mut self) -> Result<(), LoadError> {
        self.application
            .es()
            .delete::<ClassificationDto>(self.id)
            .map_err(|_| LoadError::NotFound)
    }
}

impl Classification {
    // TODO: Make name_path as ClassificationPath object
    fn set_name_path(&mut self, name_path: String) {
        self.full_path = Some(name_path)
    }
}

impl Entity for Classification {
    fn create(app: App) -> Classification {
        Classification {
            id: Uuid::new_v4(),
            full_path: None,
            application: app
        }
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

impl ToDto for Classification {
    type Dto = ClassificationDto;

    fn to_dto(&self) -> ClassificationDto {
        ClassificationDto {
            id: self.id,
            full_path: match self.full_path {
                None => panic!("Classification mast have path"),
                Some(ref s) => s.to_string(),
            },
        }
    }
}

impl FromDto for Classification {
    type Dto = ClassificationDto;

    fn from_dto(dto: Self::Dto, app: App) -> Classification {
        Classification {
            id: dto.id,
            full_path: Some(dto.full_path),
            application: app,
        }
    }
}

impl Load for Classification {
    fn load(_app: App, _id: Uuid) -> Result<Self, LoadError> {
        unimplemented!()
    }
}
