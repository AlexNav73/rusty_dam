
use uuid::Uuid;
use es::{EsClient, EsDocument};

use std::sync::{Arc, Mutex};

use {Entity, Lazy};

lazy_static! {
    static ref CONNECTION: Arc<Mutex<Connection>> = Arc::new(Mutex::new(Connection::new()));
}

pub struct Connection {
    is_logged_in: bool,
    es_client: EsClient,
}

impl Connection {
    fn new() -> Connection {
        Connection {
            is_logged_in: false,
            // TODO: Url should be stored in registration
            es_client: EsClient::new("http://localhost:9200")
                .expect("Unable to connect to elasticsearch"),
        }
    }

    pub fn get() -> Arc<Mutex<Connection>> {
        CONNECTION.clone()
    }

    pub fn login(&mut self) {
        // TODO: Proper impl
        self.is_logged_in = true;
    }

    pub fn load<T: Entity>(&mut self, id: Uuid) -> Lazy<T> {
        unimplemented!()
    }

    pub fn save<T>(&mut self, item: &T)
        where T: Entity + EsDocument
    {
        if !self.is_logged_in {
            panic!("Connection not establish. You mast Login first");
        }
        self.es_client.index(item).send();
    }
}
