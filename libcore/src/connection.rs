
use uuid::Uuid;
use dotenv::dotenv;

use std::rc::Rc;
use std::cell::RefCell;
use std::env;

use {Entity, Load, LoadError};
use es::EsService;
use pg::PgService;
use models::user::User;
use configuration::Configuration;

pub struct Connection {
    is_logged_in: bool,
    es_client: EsService,
    pg_client: PgService,
}

impl Connection {
    pub fn new<C: Configuration>(config: C) -> Connection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Connection {
            // TODO: Proper impl
            is_logged_in: true,
            es_client: EsService::new(config.es_url(), config.es_index_name()),
            pg_client: PgService::new(database_url),
        }
    }

    pub fn authorized(&self) -> bool {
        self.is_logged_in
    }

    pub fn es(&mut self) -> &mut EsService {
        &mut self.es_client
    }

    pub fn pg(&mut self) -> &mut PgService {
        &mut self.pg_client
    }
}

pub struct App {
    user: User,
    connection: Rc<RefCell<Connection>>,
}

// TODO: Remove this later ...
use super::models::pg::models::Classification;

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

    // TODO: Remove this later ...
    pub fn get_cls_by_id(&mut self, id: Uuid) -> Result<Classification,LoadError> {
        super::models::pg::get_cls_by_id(self, id)
    }

    pub fn get_name_path(&mut self, id: Uuid) -> Result<Vec<String>, LoadError> {
        super::models::pg::get_name_path(self, id)
    }
}
