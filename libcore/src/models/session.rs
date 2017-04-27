
use diesel::prelude::*;
use diesel::types::*;
use uuid::Uuid;

use std::cell::RefCell;

use LoadError;
use connection::App;

pub struct Session {
    id: Uuid,
    user_id: RefCell<Option<Uuid>>,
    login: RefCell<Option<String>>,
    application: RefCell<App>,
}

impl Session {
    pub fn new<L, P>(mut app: App, login: L, password: P) -> Result<Self, LoadError>
        where L: Into<String>,
              P: Into<String>
    {
        sql_function!(create_session,
                      create_session_t,
                      (uname: Text, upasswd: Text) -> ::diesel::pg::types::sql_types::Uuid);

        let pg_conn = app.pg().connect();
        exec_fn!(create_session(login.into(), password.into()), pg_conn)
            .and_then(|s| Ok(Session {
                id: s,
                user_id: RefCell::new(None),
                login: RefCell::new(None),
                application: RefCell::new(app)
            }))
    }

    pub fn establish<L>(mut app: App, sid: Uuid, ulogin: L) -> Result<Self, LoadError>
        where L: Into<String>
    {
        use diesel::expression::exists;
        use models::pg::schema::sessions::dsl::*;

        let log = ulogin.into();
        let pg_conn = app.pg().connect();
        ::diesel::select(exists(sessions.filter(id.eq(sid)).filter(login.eq(&log))))
            .get_result(&*pg_conn)
            .map_err(|_| LoadError::NotFound)
            .and_then(|session_exists| if session_exists {
                          Ok(Session {
                                 id: sid,
                                 user_id: RefCell::new(None),
                                 login: RefCell::new(Some(log)),
                                 application: RefCell::new(app),
                             })
                      } else {
                          Err(LoadError::NotFound)
                      })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn login(&self) -> String {
        use models::pg::schema::sessions::dsl::*;
        use models::pg::models::Session;

        if self.user_id.borrow().is_none() || self.login.borrow().is_none() {
            let mut app = self.application.borrow_mut();
            let pg_conn = app.pg().connect();

            let s = sessions
                .filter(id.eq(self.id))
                .first::<Session>(&*pg_conn)
                .unwrap();
            *self.user_id.borrow_mut() = Some(s.user_id);
            *self.login.borrow_mut() = Some(s.login);
        }

        self.login.borrow().as_ref().map(|e| e.clone()).unwrap()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        sql_function!(delete_session,
                      delete_session_t,
                      (sid: ::diesel::pg::types::sql_types::Uuid) -> Bool);

        let mut app = self.application.borrow_mut();
        let pg_conn = app.pg().connect();
        let _res: bool = exec_fn!(delete_session(self.id), pg_conn).unwrap();
    }
}
