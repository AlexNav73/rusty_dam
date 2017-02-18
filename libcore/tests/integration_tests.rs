
extern crate libcore;

use libcore::{ Uuid, Connection, ConnectionError, Record };

#[test]
fn get_record() {
    let mut c = Connection::new();
    let record: Result<Record, ConnectionError> =  c.by_id(Uuid::new_v4());
    assert!(record.is_err());
}


