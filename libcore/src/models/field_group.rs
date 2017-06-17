
use diesel::prelude::*;
use uuid::Uuid;

use {Load, SearchBy, Entity, LoadError};
use models::pg::schema::field_groups::dsl;
use models::field::Field;
use connection::App;

pub struct FieldGroupBuilder {
    name: Option<String>,
    application: App
}

impl FieldGroupBuilder {
    pub fn new(app: App) -> Self {
        FieldGroupBuilder {
            name: None,
            application: app
        }
    }

    pub fn name<S>(mut self, name: S) -> Self 
        where S: Into<String>
    {
        self.name = Some(name.into());
        self
    }

    pub fn build(mut self) -> Result<FieldGroup, LoadError> {
        use diesel::associations::HasTable;
        use models::pg::models::*;

        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let name = self.name.clone().expect("Name is not assigned");
        let new_fg = NewFieldGroup {
            name: name.as_str()
        };

        let pg_conn = self.application.pg().connect();
        ::diesel::insert(&new_fg)
            .into(dsl::field_groups::table())
            .get_result::<(Uuid, String)>(&*pg_conn)
            .map(move |fg| self::FieldGroup {
                id: fg.0,
                name: self.name.unwrap(),
                is_dirty: false,
                application: self.application,
            })
            .map_err(|_| LoadError::NotFound)
    }
}

pub struct FieldGroup {
    id: Uuid,
    name: String,
    is_dirty: bool,
    application: App
}

impl FieldGroup {
    pub fn add_field(&mut self, field: &Field) -> Result<(), LoadError> {
        use models::pg::models::*;
        use models::pg::schema::field2field_groups::dsl as f2fg;
        use diesel::associations::HasTable;

        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = self.application.pg().connect();

        let m2m = Field2FieldGroup {
            field_group_id: self.id,
            field_id: field.id(),
        };
        ::diesel::insert(&m2m)
            .into(f2fg::field2field_groups::table())
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

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

    fn update(&mut self) -> Result<(), LoadError> {
        self.is_dirty = false;
        let pg_conn = self.application.pg().connect();

        ::diesel::update(dsl::field_groups.find(self.id))
            .set(dsl::name.eq(self.name.as_str()))
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

    pub fn delete(mut self) -> Result<(), LoadError> {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = self.application.pg().connect();
        ::diesel::delete(dsl::field_groups.find(self.id))
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }
}

impl Load for FieldGroup {
    fn load(mut app: App, fid: Uuid) -> Result<Self, LoadError> {
        use models::pg::models::*;

        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = app.pg().connect();
        dsl::field_groups
            .find(fid)
            .first::<FieldGroup>(&*pg_conn)
            .map(|f| self::FieldGroup {
                id: f.id,
                name: f.name,
                is_dirty: false,
                application: app
            })
            .map_err(|_| LoadError::NotFound)
    }
}

impl SearchBy<Uuid> for FieldGroup {
    fn search(app: App, query: Uuid) -> Result<Self, LoadError> {
        FieldGroup::load(app, query)
    }
}

impl<'a> SearchBy<&'a str> for FieldGroup {
    fn search(mut app: App, fname: &'a str) -> Result<Self, LoadError> {
        use models::pg::models::*;

        let pg_conn = app.pg().connect();

        dsl::field_groups.filter(dsl::name.eq(fname))
            .first::<FieldGroup>(&*pg_conn)
            .map(|f| self::FieldGroup {
                id: f.id,
                name: f.name,
                is_dirty: false,
                application: app
            })
            .map_err(|_| LoadError::NotFound)
    }
}


impl Entity for FieldGroup {
    fn id(&self) -> Uuid {
        self.id
    }
}

