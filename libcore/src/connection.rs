
use uuid::Uuid;
use dotenv::dotenv;

use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};
use std::env;

use {Load, LoadError};
use es::EsService;
use pg::PgService;
use configuration::Configuration;
use models::session::Session;

struct Connection<'c> {
    session: Option<Session<'c>>,
    es_service: EsService,
    pg_service: PgService,
}

impl<'c> Connection<'c> {
    fn new<C: Configuration>(config: C) -> Self {
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
pub struct App<'a>(Rc<RefCell<Connection<'a>>>);

impl<'a> App<'a> {
    pub fn new<C: Configuration>(config: C) -> App<'a> {
        App(Rc::new(RefCell::new(Connection::new(config))))
    }

    pub fn login<L, P>(&mut self, login: L, password: P) -> Result<(), LoadError>
        where L: AsRef<str>,
              P: AsRef<str>
    {
        Session::new(self.clone(), login, password)
            .map(|s| (*self.0).borrow_mut().session = Some(s))
    }

    pub fn connect_to_session<L>(&mut self, id: Uuid) -> Result<(), LoadError>
    {
        Session::establish(self.clone(), id)
            .map(|s| (*self.0).borrow_mut().session = Some(s))
    }

    pub fn es<'b>(&'b mut self) -> RefMut<'b, EsService> {
        RefMut::map((*self.0).borrow_mut(), |e| e.es())
    }

    pub fn pg<'b>(&'b mut self) -> RefMut<'b, PgService> {
        RefMut::map((*self.0).borrow_mut(), |e| e.pg())
    }

    pub fn session<'b>(&'b self) -> Ref<'b, Option<Session<'a>>> {
        Ref::map((*self.0).borrow(), |e| &e.session)
    }

    pub fn get<T: Load>(&self, id: Uuid) -> Result<T, LoadError> {
        T::load(self.clone(), id)
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
