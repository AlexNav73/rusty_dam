
extern crate libcore;

use libcore::*;

struct Config;

impl Configuration for Config {
    fn id(&self) -> String {
        "".to_string()
    }

    fn es_index_name(&self) -> String {
        "rusty_dam".to_string()
    }

    fn es_url(&self) -> String {
        "http://192.168.99.100:32769".to_string()
    }
}

fn main() {
    let app = App::new(Config);
    //let record = app.create::<Record>();
    //app.save(&record).unwrap();

    let record = app.get::<Record>(Uuid::parse_str("1eb8a5e1-f1b8-439b-83ae-3d88a27dfbf8").unwrap()).unwrap();
    println!("{:?}", record.id());
}

