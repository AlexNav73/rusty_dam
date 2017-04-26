
use uuid::Uuid;
use dotenv::dotenv;

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::env;

use {Create, Load, LoadError};
use es::EsService;
use pg::PgService;
use configuration::Configuration;

struct Connection {
    es_client: EsService,
    pg_client: PgService,
}

impl Connection {
    fn new<C: Configuration>(config: C) -> Connection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Connection {
            // TODO: Proper impl
            es_client: EsService::new(config.es_url(), config.es_index_name()),
            pg_client: PgService::new(database_url),
        }
    }

    fn es(&mut self) -> &mut EsService {
        &mut self.es_client
    }

    fn pg(&mut self) -> &mut PgService {
        &mut self.pg_client
    }
}

#[derive(Clone)]
pub struct App(Rc<RefCell<Connection>>);

impl App {
    pub fn new<C: Configuration>(config: C) -> App {
        App(Rc::new(RefCell::new(Connection::new(config))))
    }

    pub fn es<'a>(&'a mut self) -> RefMut<'a, EsService> {
        RefMut::map((*self.0).borrow_mut(), |e| e.es())
    }

    pub fn pg<'a>(&'a mut self) -> RefMut<'a, PgService> {
        RefMut::map((*self.0).borrow_mut(), |e| e.pg())
    }

    pub fn get<T: Load>(&self, id: Uuid) -> Result<T, LoadError> {
        T::load(self.clone(), id)
    }

    pub fn create<T: Create>(&self) -> T {
        T::create(self.clone())
    }
}
