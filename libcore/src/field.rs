
use {Entity, Document};

use uuid::Uuid;

use std::slice::Iter;
use std::iter::FromIterator;

pub struct Field {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct FieldDto {}

impl Document<Field> for FieldDto {
    fn doc_type() -> &'static str {
        "field"
    }

    fn map(self) -> Field {
        unimplemented!()
    }
}

impl Entity for Field {
    type Dto = FieldDto;

    fn id(&self) -> Uuid {
        self.id
    }

    fn map(&self) -> FieldDto {
        unimplemented!()
    }
}

pub struct FieldCollection {
    //fields: Vec<Lazy<Field>>,
}

impl FieldCollection {
    pub fn new() -> FieldCollection {
        FieldCollection {}
    }
}

impl<'a> FromIterator<&'a Uuid> for FieldCollection {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        // FieldCollection { fields: iter.into_iter().map(|id| id.into()).collect() }
        FieldCollection {}
    }
}
