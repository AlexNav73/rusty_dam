
use uuid::Uuid;

use std::rc::Rc;
use std::cell::RefCell;

use {Entity, ToDto, FromDto, Load, LoadError};
use models::es::FieldDto;
use connection::{App, Connection};

pub struct Field {
    id: Uuid,
    name: String,
    value: FieldValue,
    connection: Rc<RefCell<Connection>>,
}

impl Field {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    // TODO: Proper impl ...
    fn value(&self) -> &FieldValue {
        &self.value
    }

    fn set_value<T: Into<FieldValue>>(&mut self, value: T) {
        self.value = value.into();
    }
}

impl Entity for Field {
    fn id(&self) -> Uuid {
        self.id
    }

    fn create(app: &App) -> Field {
        Field {
            id: Uuid::new_v4(),
            name: "".into(),
            value: FieldValue::Empty,
            connection: app.connection(),
        }
    }
}

impl ToDto for Field {
    type Dto = FieldDto;

    fn to_dto(&self) -> FieldDto {
        if self.name.is_empty() { panic!("Field name is empty"); }

        FieldDto {
            id: self.id,
            name: self.name.clone(),
            value: self.value.clone()
        }
    }
}

impl FromDto for Field {
    type Dto = FieldDto;

    fn from_dto(dto: Self::Dto, conn: Rc<RefCell<Connection>>) -> Field {
        Field {
            id: dto.id,
            name: dto.name,
            value: dto.value,
            connection: conn,
        }
    }
}

impl Load for Field {
    fn load(_c: Rc<RefCell<Connection>>, _id: Uuid) -> Result<Self, LoadError> {
        unimplemented!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FieldValue {
    Number(i64),
    Boolean(bool),
    Text(String),
    Empty
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
