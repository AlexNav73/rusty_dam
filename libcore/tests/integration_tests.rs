
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

#[test]
//#[should_panic]
fn get_record() {
    // 112a5b66-c708-4e2a-afbb-b3c600b7ec91
    // 2579847b-cb9c-4126-bffe-d7879df1198d
    // 08af83ca-62f4-44d2-9610-9a8e0c99f488
    // 838037c3-5928-4aa3-af87-ad29915cdf24
    // c8edb8a0-7f4c-49d2-8d9e-f785ec0856a3
    // 858e5f21-2802-4272-af6a-e4557a8fc999
    // da0d2d48-5d6e-4476-bb0e-82213d387e98
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let mut new_record = Record::new(app.clone()).unwrap();
        let record_id = new_record.id();
        assert!(new_record.save().is_ok());

        println!("Record id: {}", new_record.id());

        let record = app.get::<Record>(record_id);

        assert!(record.is_ok());
        println!("Record id: {}", record.unwrap().id());
    });
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
    });
}

#[test]
//#[should_panic]
fn load_cls() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let cls_id = Uuid::parse_str("f6e09bf2-4495-4047-8022-5a1317e67506").unwrap();
        let cls = app.get::<Classification>(cls_id);

        println!("Classification: {:?}", cls);

        assert!(cls.is_ok());
    });
}

#[test]
//#[should_panic]
fn load_classification_path() {
    let mut c = App::new(Config);
    c.as_admin(|app| {
        let cls_id = Uuid::parse_str("eda974ca-03b0-48a3-baf0-abec38ebc54c").unwrap();
        let cls = app.get::<Classification>(cls_id);

        println!("Classification name path: {:?}", cls);

        assert!(cls.is_ok());
    });
}

//#[test]
fn create_admin() {
    let mut c = App::new(Config);
    let u = unsafe { User::create_administrator(c, "Admin", "Admin1") };

    println!("{:?}", u);

    assert!(u.is_err());
}
