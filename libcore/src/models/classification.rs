
use diesel::prelude::*;
use uuid::Uuid;

use std::fmt;

use {Entity, ToDto, FromDto, Load, LoadError};
use models::es::ClassificationDto;
use models::pg::ClassificationNamePath;
use models::pg::schema::classifications::dsl::*;
use connection::App;

pub struct Classification {
    id: Uuid,
    parent_id: Option<Uuid>,
    name_path: ClassificationNamePath,
    is_new: bool,
    is_dirty: bool,
    application: App,
}

impl Classification {
    pub fn save(&mut self) -> Result<(), LoadError> {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        if self.is_new {
            self.save_new()
        } else if self.is_dirty {
            self.update()
        } else {
            Ok(())
        }
    }

    // TODO: Update only what was changed
    fn update(&mut self) -> Result<(), LoadError> {
        use models::pg::models::ClassificationChangeset;

        self.is_dirty = false;
        let pg_conn = self.application.pg().connect();
        let pname = self.name_path.parent();

        let pidchange = if let Some(ref p) = pname {
            Some(classifications
                     .filter(name.eq(p.name()))
                     .select(parent_id)
                     .first::<Option<Uuid>>(&*pg_conn)
                     .map_err(|_| LoadError::NotFound)?)
        } else {
            None
        };

        let changes = ClassificationChangeset {
            parent_id: pidchange,
            name: Some(self.name_path.name().to_owned()),
        };

        ::diesel::update(classifications.find(self.id))
            .set(&changes)
            .get_result::<(Uuid, Option<Uuid>, String)>(&*pg_conn)
            .map(|e| self.parent_id = e.1)
            .map_err(|_| LoadError::NotFound)
    }

    fn save_new(&mut self) -> Result<(), LoadError> {
        use diesel::associations::HasTable;
        use models::pg::models::*;

        let new_cls = NewClassification {
            id: self.id,
            parent_id: self.parent_id,
            name: self.name_path.name(),
        };
        let pg_conn = self.application.pg().connect();
        ::diesel::insert(&new_cls)
            .into(classifications::table())
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

    pub fn move_to<T: Into<ClassificationNamePath>>(&mut self,
                                                    new_path: T)
                                                    -> Result<(), LoadError> {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let mut name_path = new_path.into();
        let pg_conn = self.application.pg().connect();

        match name_path.is_valid(pg_conn) {
            Ok(r) if r == true => {
                match self.name_path.parent() {
                    Some(ref pnp) if name_path != *pnp => {
                        unsafe {
                            name_path.append_node_unchecked(self.name_path.name());
                        }
                        self.name_path = name_path;
                        self.is_dirty = true;
                        Ok(())
                    }
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }

    pub fn rename<N: Into<String>>(&mut self, new_name: N) {
        self.name_path.set_name(new_name.into());
        self.is_dirty = true;
    }

    pub fn delete(mut self) -> Result<(), LoadError> {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = self.application.pg().connect();
        ::diesel::delete(classifications.find(self.id))
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

    pub fn new<N: Into<ClassificationNamePath>>(mut app: App,
                                                name_path: N)
                                                -> Result<Self, LoadError> {
        use models::pg::models::Classification as CLS;

        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let name_path = name_path.into();
        let parent_cls = {
            let parent = name_path.parent().ok_or(LoadError::RootCls)?;
            let pg_conn = app.pg().connect();

            classifications
                .filter(name.eq(parent.name()))
                .first::<CLS>(&*pg_conn)
                .map_err(|_| LoadError::ParentNotExists)?
        };

        Ok(Classification {
               id: Uuid::new_v4(),
               parent_id: Some(parent_cls.id),
               name_path: name_path,
               is_new: true,
               is_dirty: false,
               application: app,
           })
    }
}

impl Load for Classification {
    fn load(mut app: App, cls_id: Uuid) -> Result<Self, LoadError> {
        use models::pg::models::*;

        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = app.pg().connect();
        classifications
            .find(cls_id)
            .first::<Classification>(&*pg_conn)
            .map_err(|_| LoadError::NotFound)
            .and_then(|c| {
                Ok(self::Classification {
                       id: c.id,
                       parent_id: c.parent_id,
                       name_path: ClassificationNamePath::from_uuid(pg_conn, c.id)?,
                       is_new: false,
                       is_dirty: false,
                       application: app,
                   })
            })
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

pub struct RecordClassification {
    id: Uuid,
    parent_id: Option<Uuid>,
    name_path: ClassificationNamePath,
    application: App,
}

impl Entity for RecordClassification {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl ToDto for RecordClassification {
    type Dto = ClassificationDto;

    fn to_dto(&self) -> ClassificationDto {
        ClassificationDto {
            id: self.id,
            parent_id: self.parent_id,
            name_path: self.name_path.to_string(),
        }
    }
}

impl FromDto for RecordClassification {
    type Dto = ClassificationDto;

    fn from_dto(dto: Self::Dto, app: App) -> RecordClassification {
        RecordClassification {
            id: dto.id,
            parent_id: dto.parent_id,
            name_path: dto.name_path.parse().unwrap(),
            application: app,
        }
    }
}
