
use uuid::Uuid;
use dotenv::dotenv;

use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};
use std::env;

use {SearchBy, LoadError};
use es::EsService;
use pg::PgService;
use configuration::Configuration;
use models::session::Session;

struct Connection {
    session: Option<Session>,
    es_service: EsService,
    pg_service: PgService,
}

impl Connection {
    fn new<C: Configuration>(config: C) -> Connection {
        dotenv().ok();

        // TODO: Move to config
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Connection {
            session: None,
            es_service: EsService::new(config.es_url(), config.es_index_name()),
            pg_service: PgService::new(database_url),
        }
    }

    fn es(&mut self) -> &mut EsService {
        &mut self.es_service
    }

    fn pg(&mut self) -> &mut PgService {
        &mut self.pg_service
    }
}

#[derive(Clone)]
pub struct App(Rc<RefCell<Connection>>);

impl App {
    pub fn new<C: Configuration>(config: C) -> App {
        App(Rc::new(RefCell::new(Connection::new(config))))
    }

    pub fn login<L, P>(&mut self, login: L, password: P) -> Result<(), LoadError>
        where L: AsRef<str>,
              P: AsRef<str>
    {
        Session::new(self.clone(), login, password)
            .map(|s| (*self.0).borrow_mut().session = Some(s))
    }

    pub fn connect_to_session<L>(&mut self, id: Uuid, login: L) -> Result<(), LoadError>
        where L: AsRef<str>
    {
        Session::establish(self.clone(), id, login)
            .map(|s| (*self.0).borrow_mut().session = Some(s))
    }

    pub fn es<'a>(&'a mut self) -> RefMut<'a, EsService> {
        RefMut::map((*self.0).borrow_mut(), |e| e.es())
    }

    pub fn pg<'a>(&'a mut self) -> RefMut<'a, PgService> {
        RefMut::map((*self.0).borrow_mut(), |e| e.pg())
    }

    pub fn session_id(&self) -> Option<Uuid> {
        let this = self.0.borrow();
        this.session.as_ref().map(|s| s.id().clone())
    }

    pub fn session<'a>(&'a self) -> Ref<'a, Option<Session>> {
        Ref::map((*self.0).borrow(), |e| &e.session)
    }

    pub fn get<T, Q>(&self, query: Q) -> Result<T, LoadError> 
        where T: SearchBy<Q>
    {
        T::search(self.clone(), query)
    }

    #[allow(unused_variables)]
    pub fn as_admin<F, T>(&mut self, func: F) -> Result<T, LoadError>
        where F: FnOnce(App) -> T
    {
        use std::mem::replace;

        let admin_session = Session::admin(self.clone()).ok();
        let old = replace(&mut self.0.borrow_mut().session, admin_session);

        let res = if self.0.borrow().session.is_some() {
            Ok(func(self.clone()))
        } else {
            Err(LoadError::ImpersonationFailed)
        };

        // NOTE: Result of this `replace` call MUST be assigned to variable
        // (it can't be ignored or renamed to `_`) because it will cause
        // calling `drop` on the session while it borrowed for replacing
        let admin_session = replace(&mut self.0.borrow_mut().session, old);
        res
    }
}
