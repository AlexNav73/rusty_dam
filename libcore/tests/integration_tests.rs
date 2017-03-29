
extern crate libcore;

use libcore::{Uuid, App, Record};

#[test]
fn get_record() {
    let c = App::new();
    let record = c.get::<Record>(Uuid::new_v4());
    assert!(record.is_err());
}
