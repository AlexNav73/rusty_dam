
use diesel::prelude::*;
use uuid::Uuid;

use {Load, SearchBy, Entity, ToDto, FromDto, LoadError};
use models::es::FieldDto;
use models::pg::schema::fields::dsl;
use connection::App;

pub struct FieldBuilder {
    name: Option<String>,
    application: App,
}

impl FieldBuilder {
    pub fn new(app: App) -> Self {
        FieldBuilder {
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

    pub fn build(mut self) -> Result<Field, LoadError> {
        use diesel::associations::HasTable;
        use models::pg::models::*;

        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let name = self.name.clone().expect("Name is not assigned");
        let new_field = NewField {
            name: name.as_str(),
        };

        let pg_conn = self.application.pg().connect();
        ::diesel::insert(&new_field)
            .into(dsl::fields::table())
            .get_result::<(Uuid, String)>(&*pg_conn)
            .map(move |f| self::Field {
                id: f.0,
                name: self.name.unwrap(),
                is_dirty: false,
                application: self.application,
            })
            .map_err(|_| LoadError::NotFound)
    }
}

pub struct Field {
    id: Uuid,
    name: String,
    is_dirty: bool,
    application: App,
}

impl Field {
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

        ::diesel::update(dsl::fields.find(self.id))
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
        ::diesel::delete(dsl::fields.find(self.id))
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }
}

impl Load for Field {
    fn load(mut app: App, fid: Uuid) -> Result<Self, LoadError> {
        use models::pg::models::*;

        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = app.pg().connect();
        dsl::fields
            .find(fid)
            .first::<Field>(&*pg_conn)
            .map(|f| self::Field {
                id: f.id,
                name: f.name,
                is_dirty: false,
                application: app,
            })
            .map_err(|_| LoadError::NotFound)
    }
}

impl SearchBy<Uuid> for Field {
    fn search(app: App, query: Uuid) -> Result<Self, LoadError> {
        Field::load(app, query)
    }
}

impl<'a> SearchBy<&'a str> for Field {
    fn search(mut app: App, fname: &'a str) -> Result<Self, LoadError> {
        use models::pg::models::*;

        let pg_conn = app.pg().connect();

        dsl::fields.filter(dsl::name.eq(fname))
            .first::<Field>(&*pg_conn)
            .map(|f| self::Field {
                id: f.id,
                name: f.name,
                is_dirty: false,
                application: app,
            })
            .map_err(|_| LoadError::NotFound)
    }
}

impl Entity for Field {
    fn id(&self) -> Uuid {
        self.id
    }
}

pub struct RecordField {
    id: Uuid,
    name: String,
    value: FieldValue,
    is_dirty: bool,
    application: App,
}

impl RecordField {
    pub fn empty(app: App, id: Uuid, name: String) -> RecordField {
        RecordField {
            id: id,
            name: name,
            value: FieldValue::Empty,
            is_dirty: false,
            application: app
        }
    }

    pub fn value(&self) -> &FieldValue {
        &self.value
    }

    pub fn set_value<T: Into<FieldValue>>(&mut self, value: T) {
        self.value = value.into();
        self.is_dirty = true;
    }
}

impl Entity for RecordField {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl ToDto for RecordField {
    type Dto = FieldDto;

    fn to_dto(&self) -> FieldDto {
        FieldDto {
            id: self.id,
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

impl FromDto for RecordField {
    type Dto = FieldDto;

    fn from_dto(dto: Self::Dto, app: App) -> Self {
        RecordField {
            id: dto.id,
            name: dto.name,
            value: FieldValue::Empty,
            is_dirty: false,
            application: app,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FieldValue {
    Number(i64),
    Boolean(bool),
    Text(String),
    Empty,
}

impl From<i64> for FieldValue {
    fn from(v: i64) -> FieldValue {
        FieldValue::Number(v)
    }
}

impl From<bool> for FieldValue {
    fn from(v: bool) -> FieldValue {
        FieldValue::Boolean(v)
    }
}

impl<'a> From<&'a str> for FieldValue {
    fn from(v: &'a str) -> FieldValue {
        FieldValue::Text(v.into())
    }
}

impl From<String> for FieldValue {
    fn from(v: String) -> FieldValue {
        FieldValue::Text(v)
    }
}

impl From<()> for FieldValue {
    fn from(_: ()) -> FieldValue {
        FieldValue::Empty
    }
}
