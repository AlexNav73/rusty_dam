
use uuid::Uuid;

use std::fmt;

use {Entity, ToDto, FromDto, Load, LoadError};
use models::es::ClassificationDto;
use models::pg::ClassificationNamePath;
use connection::App;

pub struct Classification {
    id: Uuid,
    parent_id: Option<Uuid>,
    name_path: ClassificationNamePath,
    application: App,
}

impl Classification {
    pub fn save(&mut self) -> Result<(), LoadError> {
        unimplemented!()
        //let dto = self.to_dto();
        //self.application
            //.es()
            //.index(&dto)
            //.map_err(|_| LoadError::NotFound)
    }

    // TODO: Delete classification from PostgreSQL too ...
    fn delete(self) -> Result<(), LoadError> {
        unimplemented!()
        //self.application
            //.es()
            //.delete::<ClassificationDto>(self.id)
            //.map_err(|_| LoadError::NotFound)
    }

    pub fn new<N: Into<ClassificationNamePath>>(app: App, name_path: N) -> Self {
        Classification {
            id: Uuid::new_v4(),
            parent_id: None,
            name_path: name_path.into(),
            application: app,
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
            parent_id: self.parent_id,
            name_path: self.name_path.to_string(),
        }
    }
}

impl FromDto for Classification {
    type Dto = ClassificationDto;

    fn from_dto(dto: Self::Dto, app: App) -> Classification {
        Classification {
            id: dto.id,
            parent_id: dto.parent_id,
            name_path: dto.name_path.parse().unwrap(),
            application: app,
        }
    }
}

impl Load for Classification {
    fn load(mut app: App, cls_id: Uuid) -> Result<Self, LoadError> {
        use diesel::prelude::*;
        use models::pg::schema::classifications::dsl::*;
        use models::pg::models::*;

        let pg_conn = app.pg().connect();
        classifications
            .filter(id.eq(cls_id))
            .first::<Classification>(&*pg_conn)
            .map_err(|_| LoadError::NotFound)
            .and_then(|c| Ok(self::Classification {
                id: c.id,
                parent_id: c.parent_id,
                name_path: ClassificationNamePath::from_uuid(&mut app, c.id)?,
                application: app
            }))
    }
}

impl fmt::Debug for Classification {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Classification")
            .field("id", &self.id)
            .field("parent_id", &self.parent_id)
            .field("name_path", &self.name_path.to_string())
            .finish()
    }
}

