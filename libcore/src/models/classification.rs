
use diesel::prelude::*;
use uuid::Uuid;

use std::fmt;

use {Definition, SearchBy, IntoEntity, Entity, ToDto, FromDto, Load, LoadError};
use models::field::RecordField;
use models::field_group::FieldGroup;
use models::es::ClassificationDto;
use models::pg::ClassificationNamePath;
use models::pg::schema::classifications::dsl::*;
use connection::App;

pub struct ClassificationBuilder {
    name: Option<String>,
    parent_id: Option<Uuid>,
    name_path: Option<ClassificationNamePath>,
    application: App,
}

impl ClassificationBuilder {
    pub fn new(app: App) -> Self {
        ClassificationBuilder {
            name: None,
            parent_id: None,
            name_path: None,
            application: app
        }
    }

    pub fn parent(mut self, pid: Uuid) -> Self {
        self.parent_id = Some(pid);
        self
    }

    pub fn name<S>(mut self, cls_name: S) -> Self 
        where S: Into<String>
    {
        self.name = Some(cls_name.into());
        self
    }

    fn name_path<T>(self, _path: T) -> Self 
        where T: Into<ClassificationNamePath>
    {
        //let name_path = name_path.into();
        //let parent_cls = {
            //let parent = name_path.parent().ok_or(LoadError::RootCls)?;
            //let pg_conn = app.pg().connect();

            //classifications
                //.filter(name.eq(parent.name()))
                //.first::<CLS>(&*pg_conn)
                //.map_err(|_| LoadError::ParentNotExists)?
        //};
        self
    }

    pub fn build(mut self) -> Result<Classification, LoadError> {
        use diesel::associations::HasTable;
        use models::pg::models::*;

        let cls_name = self.name.clone().expect("Name is not assigned");
        let new_cls = NewClassification {
            parent_id: self.parent_id,
            name: cls_name.as_str(),
        };

        let pg_conn = self.application.pg().connect();
        ::diesel::insert(&new_cls)
            .into(classifications::table())
            .get_result::<(Uuid, Option<Uuid>, String)>(&*pg_conn)
            .map(move |cls| self::Classification {
                id: cls.0,
                parent_id: cls.1,
                name_path: ClassificationNamePath::from_uuid(pg_conn, cls.0).unwrap(),
                is_dirty: false,
                application: self.application,
            })
            .map_err(|_| LoadError::NotFound)
    }
}

pub struct Classification {
    id: Uuid,
    parent_id: Option<Uuid>,
    name_path: ClassificationNamePath,
    is_dirty: bool,
    application: App,
}

impl Classification {
    pub fn save(&mut self) -> Result<(), LoadError> {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        if self.is_dirty {
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

    pub fn move_to<T>(&mut self, new_path: T) -> Result<(), LoadError> 
        where T: Into<ClassificationNamePath>
    {
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

    pub fn add_field_group(&mut self, fgroup: &FieldGroup) -> Result<(), LoadError> {
        use diesel::associations::HasTable;
        use models::pg::models::Classification2FieldGroup;
        use models::pg::schema::classification2field_groups::dsl::*;

        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = self.application.pg().connect(); 

        let m2m = Classification2FieldGroup {
            classification_id: self.id,
            field_group_id: fgroup.id()
        };
        ::diesel::insert(&m2m).into(classification2field_groups::table())
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

    pub fn get_fields(&mut self) -> Result<Vec<RecordField>, LoadError> {
        use diesel::associations::HasTable;
        use models::pg::schema::classification2field_groups::dsl::*;
        use models::pg::schema::field2field_groups::dsl::{field2field_groups, field_group_id as fgid};
        use models::pg::schema::fields::dsl::{fields, id as fid, name};

        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = self.application.pg().connect();
        let cls_fg_ids = classification2field_groups
            .filter(classification_id.eq(self.id))
            .select(field_group_id);

        field2field_groups::table()
            .inner_join(fields::table())
            .filter(fgid.eq_any(cls_fg_ids))
            .select((fid, name))
            .load::<(Uuid, String)>(&*pg_conn)
            .map(|s| s.into_iter()
                 .map(|f| RecordField::empty(self.application.clone(), f.0, f.1))
                 .collect())
            .map_err(|_| LoadError::NotFound)
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

    pub fn name(&self) -> &str {
        self.name_path.name()
    }

    pub fn id(&self) -> &Uuid {
        &self.id
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
            .map(|c| self::Classification {
                id: c.id,
                parent_id: c.parent_id,
                name_path: ClassificationNamePath::from_uuid(pg_conn, c.id).unwrap(),
                is_dirty: false,
                application: app
            })
            .map_err(|_| LoadError::NotFound)
    }
}

impl SearchBy<Uuid> for Classification {
    fn search(app: App, query: Uuid) -> Result<Self, LoadError> {
        Self::load(app, query)
    }
}

impl<'a> SearchBy<&'a str> for Classification {
    fn search(mut app: App, cls_name: &'a str) -> Result<Self, LoadError> {
        use models::pg::models::*;

        let pg_conn = app.pg().connect();

        classifications.filter(name.eq(cls_name))
            .first::<Classification>(&*pg_conn)
            .map(|c| self::Classification {
                id: c.id,
                parent_id: c.parent_id,
                name_path: ClassificationNamePath::from_uuid(pg_conn, c.id).unwrap(),
                is_dirty: false,
                application: app
            })
            .map_err(|_| LoadError::NotFound)
    }
}

impl Definition for Classification {}

impl IntoEntity<Classification> for Classification {
    fn into(self, _app: App) -> Result<Classification, LoadError> {
        Ok(self)
    }
}

impl IntoEntity<Classification> for Uuid {
    fn into(self, app: App) -> Result<Classification, LoadError> {
        Classification::load(app, self)
    }
}

impl IntoEntity<Classification> for String {
    fn into(self, mut app: App) -> Result<Classification, LoadError> {
        use models::pg::ClassificationNamePath;

        self.parse::<ClassificationNamePath>()
            .map_err(|_| LoadError::NotFound)
            .and_then(|path| path.into_uuid(app.pg().connect()))
            .and_then(|cid| Classification::load(app, cid))
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

impl From<Classification> for RecordClassification {
    fn from(cls: Classification) -> Self {
        RecordClassification {
            id: cls.id,
            parent_id: cls.parent_id,
            name_path: cls.name_path,
            application: cls.application
        }
    }
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
