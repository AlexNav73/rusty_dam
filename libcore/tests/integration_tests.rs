
extern crate libcore;

use libcore::*;

struct Config;

impl Configuration for Config {
    fn id(&self) -> String {
        "".into()
    }

    fn es_index_name(&self) -> String {
        "rusty_dam".into()
    }

    fn es_url(&self) -> String {
        "http:192.168.99.100:32769/".into()
    }
}

#[test]
//#[should_panic]
fn get_record() {
    let c = App::new(Config);
    let mut new_record = c.create::<Record>();
    let record_id = new_record.id();
    assert!(new_record.save().is_ok());

    println!("Record id: {}", new_record.id());

    let record = c.get::<Record>(record_id);

    assert!(record.is_ok());
    println!("Record id: {}", record.unwrap().id());
}

#[test]
//#[should_panic]
fn create_record() {
    let c = App::new(Config);
    let mut record = c.create::<Record>();
    let save_result = record.save();

    println!("Saved: {:?}", save_result);
    println!("Record: {}", record.id());

    let delete_result = record.delete();
    println!("Deleted: {:?}", delete_result);

    assert!(save_result.is_ok());
    assert!(delete_result.is_ok());
}

#[test]
//#[should_panic]
fn load_cls() {
    let mut c = App::new(Config);
    let cls = c.get::<Classification>(Uuid::parse_str("f6e09bf2-4495-4047-8022-5a1317e67506").unwrap());

    println!("Classification: {:?}", cls);

    assert!(cls.is_ok());
}

#[test]
//#[should_panic]
fn load_classification_path() {
    let mut c = App::new(Config);
    let cls = c.get::<Classification>(Uuid::parse_str("eda974ca-03b0-48a3-baf0-abec38ebc54c").unwrap());

    println!("Classification name path: {:?}", cls);

    assert!(cls.is_ok());
}
