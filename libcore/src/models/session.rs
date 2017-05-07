
use diesel::prelude::*;
use diesel::types::*;
use uuid::Uuid;

use std::cell::RefCell;

use LoadError;
use connection::App;

const ADMIN_USER_GROUP: &'static str = "administrators";

pub struct Session<'a> {
    id: Uuid,
    user_id: Uuid,
    login: RefCell<Option<String>>,
    application: App<'a>,
}

impl<'a> Session<'a> {
    pub fn new<L, P>(mut app: App<'a>, login: L, password: P) -> Result<Self, LoadError>
        where L: AsRef<str>,
              P: AsRef<str>
    {
        use models::pg::schema::sessions::dsl::sessions;

        sql_function!(create_session,
                      create_session_t,
                      (uname: Text, upasswd: Text) -> ::diesel::pg::types::sql_types::Uuid);

        let pg_conn = app.pg().connect();
        exec_fn!(create_session(login.as_ref(), password.as_ref()), pg_conn)
            .and_then(|s: Uuid| sessions.find(s).first::<(Uuid, Uuid)>(&*pg_conn).map_err(|_| LoadError::NotFound))
            .map(|s| self::Session {
                id: s.0,
                user_id: s.1,
                login: RefCell::new(None),
                application: app
            })
    }

    pub fn establish(mut app: App, sid: Uuid) -> Result<Self, LoadError> {
        use models::pg::schema::sessions::dsl::*;

        let pg_conn = app.pg().connect();
        sessions.find(sid)
            .first::<(Uuid, Uuid)>(&*pg_conn)
            .map_err(|_| LoadError::NotFound)
            .map(|s| Session {
                id: s.0,
                user_id: s.1,
                login: RefCell::new(None),
                application: app,
            })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn admin(mut app: App) -> Result<Self, LoadError> {
        use diesel::associations::HasTable;
        use models::pg::schema::users::dsl::{users, id};
        use models::pg::schema::user_group::dsl::{user_group, name};
        use models::pg::schema::user2user_group::dsl::{user2user_group, user_id};
        use models::pg::schema::sessions::dsl::sessions;
        use models::pg::models::*;

        let pg_conn = app.pg().connect();

        let admin_users_id = user_group::table()
            .inner_join(user2user_group::table())
            .filter(name.eq(ADMIN_USER_GROUP))
            .select(user_id);
        let u = users.filter(id.eq_any(admin_users_id)).select(id)
            .first::<Uuid>(&*pg_conn)
            .map_err(|_| LoadError::NotFound)?;

        let s = Session { user_id: u };

        ::diesel::insert(&s).into(sessions::table())
            .get_result::<(Uuid, Uuid)>(&*pg_conn)
            .map(|e| self::Session {
                id: e.0,
                user_id: e.1,
                login: RefCell::new(None),
                application: app
            })
            .map_err(|_| LoadError::NotFound)
    }

    pub fn login(&self) -> &str {
        use diesel::associations::HasTable;
        use models::pg::schema::sessions::dsl::{sessions, user_id};
        use models::pg::schema::users::dsl::{users, id, login};
        use models::pg::models::*;

        let pg_conn = self.application.pg().connect();
        if let Some(ref name) = *self.login.borrow() {
            name
        } else {
            let name = users.filter(id.eq_any(sessions.find(self.id).select(user_id)))
                .select(login)
                .first::<String>(&*pg_conn)
                .unwrap(); // FIXME: Discard connection errors for now

            *self.login.borrow_mut() = Some(name);
            self.login.borrow().as_ref().unwrap()
        }
    }
}

impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        sql_function!(delete_session,
                      delete_session_t,
                      (sid: ::diesel::pg::types::sql_types::Uuid) -> Bool);

        let pg_conn = self.application.pg().connect();
        let _res: bool = exec_fn!(delete_session(self.id), pg_conn).unwrap();
    }
}
