
extern crate libcore;

use libcore::{ Uuid, Connection, ConnectionError, Record };

fn main() {
    let mut c = Connection::new();
    //let record: Result<Record, ConnectionError> =  c.by_id(Uuid::new_v4());
}
