
use uuid::Uuid;
use rs_es::operations::get::GetResult;

use es::EsClient;

use std::sync::{Arc, Mutex};

use {Entity, Document};

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

    pub fn login(&mut self) {
        // TODO: Proper impl
        self.is_logged_in = true;
    }

    pub fn by_id<T: Entity>(&mut self, id: Uuid) -> Result<T, ConnectionError> {
        match self.es_client.find_by_id::<T>(id).send() {
            Ok(GetResult { source: Some(doc), .. }) => {
                let doc: T::Dto = doc;
                Ok(doc.map())
            },
            _ => Err(ConnectionError::NotFound)
        }
    }

    pub fn save<T>(&mut self, item: &T) where T: Entity {
        if !self.is_logged_in {
            panic!("Connection not establish. You mast Login first");
        }
        self.es_client.index(item).send();
    }
}

// TODO: Rename
pub enum ConnectionError {
    NotFound
}

