
use uuid::Uuid;
use rs_es::operations::get::GetResult;

use es::EsClient;

use std::sync::{Arc, Mutex};

use {Entity, Lazy, Document};

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

    pub fn load<T: Entity, U: Document<T>>(&mut self, id: Uuid) -> Result<Lazy<T>, ConnectionError> {
        match self.es_client.find_by_id::<T, U>(id).send() {
            Ok(GetResult { source: Some(doc), .. }) => {
                let doc: U = doc;
                let doc: T = doc.map();
                Ok(doc.into())
            },
            _ => Err(ConnectionError::NotFound)
        }
    }

    pub fn save<T, U>(&mut self, item: &T) 
        where T: Entity + Document<U>,
              U: Entity
    {
        if !self.is_logged_in {
            panic!("Connection not establish. You mast Login first");
        }
        self.es_client.index(item).send();
    }
}

// TODO: Rename
enum ConnectionError {
    NotFound
}

