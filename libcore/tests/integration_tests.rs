
extern crate libcore;

use libcore::*;

struct Config;

impl Configuration for Config {
    fn id(&self) -> String { "".into() }

    fn es_index_name(&self) -> String {
        "rusty_dam".into()
    }

    fn es_url(&self) -> String {
        "http://192.168.99.100:32769/".into()
    }
}

#[test]
fn get_record() {
    let c = App::new(Config);
    let record = c.get::<Record>(Uuid::new_v4());
    assert!(record.is_err());
}

#[test]
fn create_record() {
    let c = App::new(Config);
    let mut record = c.create::<Record>();
    assert!(record.save().is_ok());
    assert!(record.delete().is_ok());
}

#[test]
fn load_cls() {
    let mut c = App::new(Config);
    let cls = c.get_cls_by_id(Uuid::parse_str("0f6c2408-8e68-4c75-a14f-1c84b63868c6").unwrap());
    assert!(cls.is_ok());
}

#[test]
fn load_classification_path() {
    let mut c = App::new(Config);
    let cls = c.get_name_path(Uuid::parse_str("0f6c2408-8e68-4c75-a14f-1c84b63868c6").unwrap());
    println!("{:?}", cls);
    assert!(false);
}
