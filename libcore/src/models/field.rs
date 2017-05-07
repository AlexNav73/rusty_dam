
use diesel::prelude::*;
use uuid::Uuid;

use {Load, Entity, ToDto, FromDto, LoadError};
use models::es::FieldDto;
use models::pg::schema::fields::dsl;
use connection::App;

pub struct Field<'a> {
    id: Uuid,
    name: String,
    is_new: bool,
    is_dirty: bool,
    application: App<'a>,
}

impl<'a> Field<'a> {
    pub fn new<S: Into<String>>(app: App, fname: S) -> Result<Self, LoadError> {
        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        Ok(Field {
               id: Uuid::new_v4(),
               name: fname.into(),
               is_new: true,
               is_dirty: false,
               application: app,
           })
    }

    pub fn add_to_field_group(&mut self, fg_id: Uuid) -> Result<(), LoadError> {
        use models::pg::models::*;
        use models::pg::schema::field_groups::dsl::*;
        use models::pg::schema::field2field_groups::dsl as f2fg;
        use diesel::associations::HasTable;

        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = self.application.pg().connect();
        field_groups.find(fg_id).select(id)
            .first::<Uuid>(&*pg_conn)
            .and_then(|fid| {
                let m2m = Field2FieldGroup {
                    field_id: self.id,
                    field_group_id: fid,
                };
                ::diesel::insert(&m2m)
                    .into(f2fg::field2field_groups::table())
                    .execute(&*pg_conn)
            })
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

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

    fn update(&mut self) -> Result<(), LoadError> {
        self.is_dirty = false;
        let pg_conn = self.application.pg().connect();

        ::diesel::update(dsl::fields.find(self.id))
            .set(dsl::name.eq(self.name.as_str()))
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

    fn save_new(&mut self) -> Result<(), LoadError> {
        use diesel::associations::HasTable;
        use models::pg::models::*;

        self.is_new = false;
        let new_field = NewField {
            id: self.id,
            name: self.name.as_str(),
        };

        let pg_conn = self.application.pg().connect();
        ::diesel::insert(&new_field)
            .into(dsl::fields::table())
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

impl<'a> Load for Field<'a> {
    fn load(mut app: App, fid: Uuid) -> Result<Self, LoadError> {
        use models::pg::models::*;

        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = app.pg().connect();
        dsl::fields
            .find(fid)
            .first::<Field>(&*pg_conn)
            .map_err(|_| LoadError::NotFound)
            .and_then(|f| {
                Ok(self::Field {
                       id: f.id,
                       name: f.name,
                       is_new: false,
                       is_dirty: false,
                       application: app,
                   })
            })
    }
}

pub struct RecordField<'a> {
    id: Uuid,
    name: String,
    value: FieldValue,
    is_dirty: bool,
    application: App<'a>,
}

impl<'a> RecordField<'a> {
    pub fn empty(app: App, id: Uuid, name: String) -> Self {
        RecordField {
            id: id,
            name: name,
            value: FieldValue::Empty,
            is_dirty: false,
            application: app
        }
    }

    fn value(&self) -> &FieldValue {
        &self.value
    }

    fn set_value<T: Into<FieldValue>>(&mut self, value: T) {
        self.value = value.into();
        self.is_dirty = true;
    }
}

impl<'a> Entity for RecordField<'a> {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl<'a> ToDto<'a> for RecordField<'a> {
    type Dto = FieldDto<'a>;

    fn to_dto(&self) -> Self::Dto {
        FieldDto {
            id: self.id,
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

impl<'a, 'b> FromDto<'a> for RecordField<'b> {
    type Dto = FieldDto<'a>;

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
