
extern crate libcore;

use libcore::Uuid;
use libcore::App;
use libcore::Configuration;
use libcore::Entity;
use libcore::LoadError;

use libcore::record::Record;
use libcore::classification::Classification;
use libcore::user::User;
use libcore::field::{Field, FieldBuilder};
use libcore::field_group::{FieldGroup, FieldGroupBuilder};

struct Config;

impl Configuration for Config {
    fn id(&self) -> String {
        "".into()
    }

    fn es_index_name(&self) -> String {
        "rusty_dam".into()
    }

    fn es_url(&self) -> String {
        "http://192.168.99.100:32769/".into()
    }
}

#[test]
//#[should_panic]
fn get_record() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let mut new_record = Record::new(app.clone()).unwrap();
        let record_id = new_record.id();

        println!("Record id: {}", new_record.id());
        assert!(new_record.save().is_ok());

        let record = app.get(record_id)
            .map(|r: Record| println!("Record id: {}", r.id()));

        assert!(record.is_ok());
    }).unwrap();
}

#[test]
//#[should_panic]
fn create_record() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let mut record = Record::new(app).unwrap();
        let save_result = record.save();

        println!("Saved: {:?}", save_result);
        println!("Record: {}", record.id());

        let delete_result = record.delete();
        println!("Deleted: {:?}", delete_result);

        assert!(save_result.is_ok());
        assert!(delete_result.is_ok());
    }).unwrap();
}

#[test]
//#[should_panic]
fn load_cls() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let cls: Result<Classification, LoadError> = app.get("Armani");

        println!("Classification: {:?}", cls);

        assert!(cls.is_ok());
    }).unwrap();
}

#[test]
//#[should_panic]
fn load_classification_path() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let cls: Result<Classification, LoadError> = app.get("Armani");

        println!("Classification name path: {:?}", cls);

        assert!(cls.is_ok());
    }).unwrap();
}

//#[test]
fn create_admin() {
    let mut c = App::new(Config);
    let u = unsafe { User::create_administrator(c, "Admin", "Admin1") };

    println!("{:?}", u);

    assert!(u.is_err());
}

#[test]
fn add_field_to_classification() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let mut gender: Field = app.get("Gender")
            .or_else(|_| FieldBuilder::new(app.clone())
                            .name("Gender")
                            .build())
            .expect("Unauthorized access");
        gender.save().expect("Can't save field");

        let mut field_group: FieldGroup = app.get("Basic")
            .or_else(|_| FieldGroupBuilder::new(app.clone())
                            .name("Basic")
                            .build())
            .expect("Unauthorized access");
        field_group.save().expect("Can't save field group");

        let mut armani_cls: Classification = app.get("Armani")
            .expect("Classification not found");

        let _ = field_group.add_field(&gender);
        let _ = armani_cls.add_field_group(&field_group);

        let mut record = Record::new(app).unwrap();
        let save_result = record.save();

        record.classify_as(armani_cls).expect("Can't classify record with this classification");
        record.fields_mut()[&gender.id()].set_value("Man");
        record.save().expect("Record not saved");
    }).unwrap();
}
