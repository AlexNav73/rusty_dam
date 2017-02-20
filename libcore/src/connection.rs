
use uuid::Uuid;

use es::EsRepository;

use std::sync::{Arc, Mutex};

use {Entity, Document};

pub struct Connection {
    is_logged_in: bool,
    es_client: EsRepository,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            is_logged_in: false,
            // TODO: Url should be stored in registration
            es_client: EsRepository::new("http://localhost:9200")
        }
    }

    pub fn login(&mut self) {
        // TODO: Proper impl
        self.is_logged_in = true;
    }

    pub fn by_id<T: Entity>(&mut self, id: Uuid) -> Result<T, ConnectionError> {
        self.es_client.get::<T>(id).map_err(|e| ConnectionError::NotFound)
    }

    pub fn save<T: Entity>(&mut self, item: &T) {
        if !self.is_logged_in {
            panic!("Connection not establish. You mast Login first");
        }
        self.es_client.index(item);
    }
}

// TODO: Rename
pub enum ConnectionError {
    NotFound
}

