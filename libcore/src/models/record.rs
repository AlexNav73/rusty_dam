
use uuid::Uuid;
use chrono::{DateTime, UTC};

use {IntoEntity, Load, LoadError, Entity, ToDto, FromDto};
use es::SystemInfo;
use connection::App;

use models::es::RecordDto;
use models::file::File;
use models::field::RecordField;
use models::classification::{Classification, RecordClassification};
use models::collections::EntityCollection;
use models::collections::fields::FieldCollection;
use models::collections::files::FileCollection;
use models::collections::classifications::ClassificationCollection;

pub struct Record<'a> {
    id: Uuid,
    fields: FieldCollection<'a>,
    classifications: ClassificationCollection<'a>,
    files: FileCollection<'a>,
    created_by: String,
    created_on: DateTime<UTC>,
    modified_by: String,
    modified_on: DateTime<UTC>,
    is_new: bool,
    application: App<'a>,
}

impl<'a> Record<'a> {
    pub fn new(app: App) -> Result<Record, LoadError> {
        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let login = app.session()
            .as_ref()
            .map(|s| s.login().to_owned())
            .unwrap();

        Ok(Record {
               id: Uuid::new_v4(),
               fields: FieldCollection::new(app.clone()),
               classifications: ClassificationCollection::new(app.clone()),
               files: FileCollection::new(app.clone()),
               created_on: UTC::now(),
               modified_on: UTC::now(),
               created_by: login.clone(),
               modified_by: login,
               is_new: true,
               application: app.clone(),
           })
    }

    pub fn delete(mut self) -> Result<(), LoadError> {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        self.application
            .es()
            .delete::<RecordDto>(self.id)
            .map_err(|_| LoadError::NotFound)
    }

    pub fn save(&mut self) -> Result<(), LoadError> {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        if self.is_new {
            self.is_new = false;
            let dto = self.to_dto();
            self.application
                .es()
                .index(&dto)
                .map_err(|_| LoadError::NotFound)
        } else {
            self.update()
        }
    }

    fn update(&mut self) -> Result<(), LoadError> {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        // TODO: For now, just reindex whole document
        self.application
            .es()
            .delete::<RecordDto>(self.id)
            .map_err(|_| LoadError::NotFound)?;

        let dto = self.to_dto();
        self.application
            .es()
            .index(&dto)
            .map_err(|_| LoadError::NotFound)
    }

    pub fn fields(&self) -> &FieldCollection {
        &self.fields
    }

    pub fn classify_as<T>(&mut self, cls: T) -> Result<(), LoadError>
        where T: IntoEntity<Classification<'a>>
    {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let mut classification = cls.into(self.application.clone())?;
        classification.get_fields()
            .map(|r| for f in r { self.fields.add(f) })
            .map(|_| self.classifications.add(RecordClassification::from(classification)))
    }
}

impl<'a> Entity for Record<'a> {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl<'a> ToDto<'a> for Record<'a> {
    type Dto = RecordDto<'a>;

    fn to_dto(&self) -> Self::Dto {
        RecordDto {
            fields: self.fields.iter().map(|x| x.to_dto()).collect(),
            classifications: self.classifications.iter().map(|x| x.to_dto()).collect(),
            files: self.files.iter().map(|x| x.to_dto()).collect(),
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

impl<'a, 'b> FromDto<'a> for Record<'b> {
    type Dto = RecordDto<'a>;

    fn from_dto(dto: Self::Dto, app: App) -> Record<'b> {
        Record {
            id: dto.system.id,
            fields: FieldCollection::from_iter(dto.fields
                                                  .into_iter()
                                                  .map(|x| RecordField::from_dto(x, app.clone())),
                                               app.clone()),
            classifications: ClassificationCollection::from_iter(dto.classifications
                                                                    .into_iter()
                                                                    .map(|x| RecordClassification::from_dto(x, app.clone())),
                                                                 app.clone()),
            files: FileCollection::from_iter(dto.files
                                                .into_iter()
                                                .map(|x| File::from_dto(x, app.clone())),
                                             app.clone()),
            created_by: dto.system.created_by.to_string(),
            created_on: DateTime::from_utc(dto.system.created_on, UTC),
            modified_by: dto.system.modified_by.to_string(),
            modified_on: DateTime::from_utc(dto.system.modified_on, UTC),
            is_new: false,
            application: app,
        }
    }
}

impl<'a> Load for Record<'a> {
    fn load(mut app: App, id: Uuid) -> Result<Self, LoadError> {
        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let app_cloned = app.clone();
        app.es()
            .get(app_cloned, id)
            .map_err(|_| LoadError::NotFound)
    }
}
