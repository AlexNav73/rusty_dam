
use uuid::Uuid;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use LoadError;
use models::pg::schema::*;

trait PgDto {}

struct PgClient {
    connection: PgConnection,
}

impl PgClient {
    fn new(url: String) -> PgClient {
        PgClient {
            connection: PgConnection::establish(&url).expect(&format!("Error connecting to {}",
                                                                      url)),
        }
    }

    fn load<T: PgDto>(&self, _id: Uuid) -> Result<T, LoadError> {
        unimplemented!()
    }
}

pub struct PgService;

impl PgService {
    pub fn new(_url: String) -> PgService {
        PgService
    }
}
