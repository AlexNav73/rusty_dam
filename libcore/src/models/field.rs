
use diesel::prelude::*;
use uuid::Uuid;

use {Entity, ToDto, FromDto, Load, LoadError};
use models::es::FieldDto;
use models::pg::schema::fields::dsl::*;
use connection::App;

pub struct Field {
    id: Uuid,
    name: String,
    value: FieldValue,
    is_new: bool,
    is_dirty: bool,
    application: App,
}

impl Field {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn new<S: Into<String>>(app: App, fname: S) -> Self {
        Field {
            id: Uuid::new_v4(),
            name: fname.into(),
            value: FieldValue::Empty,
            is_new: true,
            is_dirty: false,
            application: app,
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    // TODO: Proper impl ...
    fn value(&self) -> &FieldValue {
        &self.value
    }

    fn set_value<T: Into<FieldValue>>(&mut self, value: T) {
        self.value = value.into();
    }

    pub fn save(&mut self) -> Result<(), LoadError> {
        if self.is_new {
            self.save_new()
        } else if self.is_dirty {
            self.update()
        } else {
            Ok(())
        }
    }

    fn update(&mut self) -> Result<(), LoadError> {
        let pg_conn = self.application.pg().connect();

        ::diesel::update(fields.filter(id.eq(self.id)))
            .set(name.eq(self.name.as_str()))
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

    fn save_new(&mut self) -> Result<(), LoadError> {
        use diesel::associations::HasTable;
        use models::pg::models::*;

        let new_field = NewField {
            id: self.id,
            name: self.name.as_str(),
        };

        let pg_conn = self.application.pg().connect();
        ::diesel::insert(&new_field)
            .into(fields::table())
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }
}

impl Entity for Field {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl ToDto for Field {
    type Dto = FieldDto;

    fn to_dto(&self) -> FieldDto {
        if self.name.is_empty() {
            panic!("Field name is empty");
        }

        FieldDto {
            id: self.id,
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

impl FromDto for Field {
    type Dto = FieldDto;

    fn from_dto(dto: Self::Dto, app: App) -> Field {
        Field {
            id: dto.id,
            name: dto.name,
            value: dto.value,
            is_new: false,
            is_dirty: false,
            application: app,
        }
    }
}

impl Load for Field {
    fn load(_app: App, _id: Uuid) -> Result<Self, LoadError> {
        unimplemented!()
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
