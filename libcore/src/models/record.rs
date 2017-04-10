
use uuid::Uuid;
use chrono::{DateTime, UTC};

use std::rc::Rc;
use std::cell::RefCell;

use {Load, LoadError, Entity, ToDto, FromDto};
use es::SystemInfo;
use connection::{App, Connection};

use models::es::RecordDto;
use models::collections::EntityCollection;
use models::collections::fields::FieldCollection;
use models::collections::files::FileCollection;
use models::collections::classifications::ClassificationCollection;

pub struct Record {
    id: Uuid,
    fields: RefCell<FieldCollection>,
    classifications: RefCell<ClassificationCollection>,
    files: RefCell<FileCollection>,
    created_by: String,
    created_on: DateTime<UTC>,
    modified_by: String,
    modified_on: DateTime<UTC>,
    is_new: bool,
    connection: Rc<RefCell<Connection>>,
}

impl Entity for Record {
    fn id(&self) -> Uuid {
        self.id
    }

    fn create(app: &App) -> Record {
        Record {
            id: Uuid::new_v4(),
            fields: RefCell::new(FieldCollection::new(app.connection())),
            classifications: RefCell::new(ClassificationCollection::new(app.connection())),
            files: RefCell::new(FileCollection::new(app.connection())),
            created_on: UTC::now(),
            modified_on: UTC::now(),
            created_by: app.user().login().to_string(),
            modified_by: app.user().login().to_string(),
            is_new: true,
            connection: app.connection(),
        }
    }
}

impl ToDto for Record {
    type Dto = RecordDto;

    fn to_dto(&self) -> RecordDto {
        RecordDto {
            fields: to_dto_collection(&mut *self.fields.borrow_mut()),
            classifications: to_dto_collection(&mut *self.classifications.borrow_mut()),
            files: to_dto_collection(&mut *self.files.borrow_mut()),
            system: SystemInfo {
                id: self.id,
                created_by: self.created_by.to_string(),
                created_on: self.created_on.naive_utc(),
                modified_by: self.modified_by.to_string(),
                modified_on: self.modified_on.naive_utc(),
            },
        }
    }
}

fn to_dto_collection<T: Load, C: EntityCollection<T>>(collection: &mut C)
                                                      -> Vec<<T as ToDto>::Dto> {
    collection
        .iter_mut()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().to_dto())
        .collect()
}

impl FromDto for Record {
    type Dto = RecordDto;

    fn from_dto(dto: Self::Dto, conn: Rc<RefCell<Connection>>) -> Record {
        Record {
            id: dto.system.id,
            fields: RefCell::new(FieldCollection::from_iter(dto.fields.into_iter().map(|x| x.id),
                                                            conn.clone())),
            classifications:
                RefCell::new(ClassificationCollection::from_iter(dto.classifications
                                                                     .into_iter()
                                                                     .map(|x| x.id),
                                                                 conn.clone())),
            files: RefCell::new(FileCollection::from_iter(dto.files.into_iter().map(|x| x.id),
                                                          conn.clone())),
            created_by: dto.system.created_by.to_string(),
            created_on: DateTime::from_utc(dto.system.created_on, UTC),
            modified_by: dto.system.modified_by.to_string(),
            modified_on: DateTime::from_utc(dto.system.modified_on, UTC),
            is_new: false,
            connection: conn,
        }
    }
}

impl Load for Record {
    fn load(c: Rc<RefCell<Connection>>, id: Uuid) -> Result<Self, LoadError> {
        c.borrow_mut()
            .es()
            .by_id(c.clone(), id)
            .map_err(|_| LoadError::NotFound)
    }
}
