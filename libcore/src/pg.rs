
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::Connection as Conn;

struct PgClient {
    connection: PgConnection
}

impl PgClient {
    fn new(url: String) -> PgClient {
        PgClient {
            connection: PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
        }
    }
}

pub struct PgService;

impl PgService {
    pub fn new(_url: String) -> PgService {
        PgService
    }
}

