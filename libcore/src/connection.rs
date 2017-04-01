
use uuid::Uuid;

use std::rc::Rc;
use std::cell::RefCell;

use {Entity, Load, LoadError};
use es::{EsService, EsError, EsDto};
use models::user::User;
use configuration::Configuration;

pub struct Connection {
    is_logged_in: bool,
    es_client: EsService,
}

impl Connection {
    pub fn new<C: Configuration>(config: C) -> Connection {
        Connection {
            // TODO: Proper impl
            is_logged_in: true,
            es_client: EsService::new(config.es_url(), config.es_index_name()),
        }
    }

    pub fn authorized(&self) -> bool {
        self.is_logged_in
    }

    pub fn es(&mut self) -> &mut EsService {
        &mut self.es_client
    }

    pub fn save<T: EsDto>(conn: Rc<RefCell<Connection>>, item: &T) -> Result<(), EsError> {
        if !conn.borrow().authorized() {
            panic!("Connection not establish. You mast Login first");
        }
        conn.borrow_mut().es_client.index(item)
    }
}

pub struct App {
    user: User,
    connection: Rc<RefCell<Connection>>,
}

impl App {
    pub fn new<C: Configuration>(config: C) -> App {
        let connection = Rc::new(RefCell::new(Connection::new(config)));
        App {
            user: User::get(connection.clone()),
            connection: connection,
        }
    }

    pub fn connection(&self) -> Rc<RefCell<Connection>> {
        self.connection.clone()
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn get<T: Load>(&self, id: Uuid) -> Result<T, LoadError> {
        T::load(self.connection(), id)
    }

    pub fn create<T: Entity>(&self) -> T {
        T::create(self)
    }

    pub fn save<T: EsDto>(&self, item: &T) -> Result<(), EsError> {
        Connection::save(self.connection(), item)
    }
}
