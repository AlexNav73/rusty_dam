
use uuid::Uuid;

use std::rc::Rc;
use std::cell::RefCell;

use es::{EsRepository, EsError};

use Entity;

pub struct Connection {
    is_logged_in: bool,
    es_client: EsRepository,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            is_logged_in: false,
            // TODO: Url should be stored in registration
            es_client: EsRepository::new("http://localhost:9200"),
        }
    }

    pub fn login(&mut self) {
        // TODO: Proper impl
        self.is_logged_in = true;
    }

    pub fn authorized(&self) -> bool {
        self.is_logged_in
    }

    pub fn by_id<T: Entity>(conn: Rc<RefCell<Connection>>, id: Uuid) -> Result<T, EsError> {
        let mut this = conn.borrow_mut();
        this.es_client.get::<T>(conn.clone(), id).map_err(|_| EsError::NotFound)
    }

    pub fn save<T: Entity>(conn: Rc<RefCell<Connection>>, item: &T) {
        if !conn.borrow().authorized() {
            panic!("Connection not establish. You mast Login first");
        }
        conn.borrow_mut().es_client.index(item);
    }
}

pub struct App(Rc<RefCell<Connection>>);

impl App {
    fn new() -> App {
        App(Rc::new(RefCell::new(Connection::new())))
    }

    fn get<T: Entity>(&self, id: Uuid) -> Result<T, EsError> {
        Connection::by_id(self.0.clone(), id)
    }

    fn save<T: Entity>(&self, item: &T) {
        Connection::save(self.0.clone(), item)
    }
}
