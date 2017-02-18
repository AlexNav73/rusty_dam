
use uuid::Uuid;

use std::slice::Iter;
use std::iter::FromIterator;

use {Entity, Document};

pub struct Classification {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct ClassificationDto {}

impl Document<Classification> for ClassificationDto {
    fn doc_type() -> &'static str {
        "classification"
    }

    fn map(self) -> Classification {
        unimplemented!()
    }
}

impl Entity for Classification {
    type Dto = ClassificationDto;

    fn id(&self) -> Uuid {
        self.id
    }

    fn map(&self) -> ClassificationDto {
        unimplemented!()
    }
}

pub struct ClassificationCollection {
    //classifications: Vec<Lazy<Classification>>,
}

impl ClassificationCollection {
    pub fn new() -> ClassificationCollection {
        ClassificationCollection {}
    }
}

impl<'a> FromIterator<&'a Uuid> for ClassificationCollection {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        //ClassificationCollection { classifications: iter.into_iter().map(|id| id.into()).collect() }
        ClassificationCollection {}
    }
}
