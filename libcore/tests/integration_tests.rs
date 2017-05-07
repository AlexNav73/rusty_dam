
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
        "http://192.168.99.100:32769/".into()
    }
}

//#[test]
//#[should_panic]
fn get_record() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let mut new_record = Record::new(app.clone()).unwrap();
        let record_id = new_record.id();

        println!("Record id: {}", new_record.id());
        assert!(new_record.save().is_ok());

        let record = app.get::<Record>(record_id);

        println!("Record id: {}", record.as_ref().map(|r| r.id()).unwrap());
        assert!(record.is_ok());
    }).unwrap();
}

//#[test]
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
fn assign_classification_to_record() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let mut record = Record::new(app.clone()).unwrap();
        let cls = app.get::<Classification>(Uuid::parse_str("025399e4-3484-4ade-9edb-cd0feb2a19a6").unwrap()).unwrap();
        record.classify_as(cls);
        let save_result = record.save();

        println!("Saved: {:?}", save_result);
        println!("Record: {}", record.id());

        //let delete_result = record.delete();
        //println!("Deleted: {:?}", delete_result);

        assert!(save_result.is_ok());
        //assert!(delete_result.is_ok());
    }).unwrap();
}

//#[test]
//#[should_panic]
fn load_cls() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let cls_id = Uuid::parse_str("f6e09bf2-4495-4047-8022-5a1317e67506").unwrap();
        let cls = app.get::<Classification>(cls_id);

        println!("Classification: {:?}", cls);

        assert!(cls.is_ok());
    }).unwrap();
}

//#[test]
//#[should_panic]
fn load_classification_path() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let cls_id = Uuid::parse_str("eda974ca-03b0-48a3-baf0-abec38ebc54c").unwrap();
        let cls = app.get::<Classification>(cls_id);

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
