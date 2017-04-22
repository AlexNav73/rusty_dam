
use uuid::Uuid;
use chrono::{DateTime, UTC};

use std::cell::RefCell;

use {Create, Load, LoadError, Entity, ToDto, FromDto};
use es::SystemInfo;
use connection::App;

use models::es::RecordDto;
use models::file::File;
use models::field::Field;
use models::classification::Classification;
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
    application: App,
}

impl Record {
    pub fn delete(mut self) -> Result<(), LoadError> {
        self.application
            .es()
            .delete::<RecordDto>(self.id)
            .map_err(|_| LoadError::NotFound)
    }

    pub fn save(&mut self) -> Result<(), LoadError> {
        let dto = self.to_dto();
        self.application
            .es()
            .index(&dto)
            .map_err(|_| LoadError::NotFound)
    }
}

impl Entity for Record {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Create for Record {
    fn create(app: App) -> Record {
        Record {
            id: Uuid::new_v4(),
            fields: RefCell::new(FieldCollection::new(app.clone())),
            classifications: RefCell::new(ClassificationCollection::new(app.clone())),
            files: RefCell::new(FileCollection::new(app.clone())),
            created_on: UTC::now(),
            modified_on: UTC::now(),
            created_by: app.user().login().to_string(),
            modified_by: app.user().login().to_string(),
            is_new: true,
            application: app.clone(),
        }
    }
}

impl ToDto for Record {
    type Dto = RecordDto;

    fn to_dto(&self) -> RecordDto {
        let classifications = to_dto_collection(&mut *self.classifications.borrow_mut());
        let files = to_dto_collection(&mut *self.files.borrow_mut());

        assert!(classifications.is_empty(),
                "Record must be assign at least one classification");
        assert!(files.is_empty(), "Record must contains at least one file");

        RecordDto {
            fields: to_dto_collection(&mut *self.fields.borrow_mut()),
            classifications: classifications,
            files: files,
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

fn to_dto_collection<T, C>(collection: &mut C) -> Vec<<T as ToDto>::Dto>
    where C: EntityCollection<T>,
          T: Load
{
    collection
        // TODO: Load all a once ... 
        .iter_mut()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().to_dto())
        .collect()
}

impl FromDto for Record {
    type Dto = RecordDto;

    fn from_dto(dto: Self::Dto, app: App) -> Record {
        Record {
            id: dto.system.id,
            fields: RefCell::new(FieldCollection::from_iter(dto.fields.into_iter().map(|x| Field::from_dto(x, app.clone())),
                                                            app.clone())),
            classifications:
                RefCell::new(ClassificationCollection::from_iter(dto.classifications
                                                                     .into_iter()
                                                                     .map(|x| Classification::from_dto(x, app.clone())),
                                                                 app.clone())),
            files: RefCell::new(FileCollection::from_iter(dto.files.into_iter().map(|x| File::from_dto(x, app.clone())),
                                                          app.clone())),
            created_by: dto.system.created_by.to_string(),
            created_on: DateTime::from_utc(dto.system.created_on, UTC),
            modified_by: dto.system.modified_by.to_string(),
            modified_on: DateTime::from_utc(dto.system.modified_on, UTC),
            is_new: false,
            application: app,
        }
    }
}

impl Load for Record {
    fn load(mut app: App, id: Uuid) -> Result<Self, LoadError> {
        let app_cloned = app.clone();
        app.es()
            .by_id(app_cloned, id)
            .map_err(|_| LoadError::NotFound)
    }
}
