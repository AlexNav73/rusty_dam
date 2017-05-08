
use diesel::prelude::*;
use diesel::types::*;
use uuid::Uuid;

use LoadError;
use models::get_sha3;
use connection::App;

const ADMIN_USER_GROUP: &'static str = "administrators";

pub struct Session {
    id: Uuid,
    user_id: Uuid,
    login: String,
    application: App,
}

impl Session {
    pub fn new<L, P>(mut app: App, login: L, password: P) -> Result<Self, LoadError>
        where L: AsRef<str>,
              P: AsRef<str>
    {
        use models::pg::schema::sessions::dsl::sessions;

        sql_function!(create_session,
                      create_session_t,
                      (uname: Text, upasswd: Text) -> ::diesel::pg::types::sql_types::Uuid);

        let pg_conn = app.pg().connect();
        let pass = get_sha3(password);
        exec_fn!(create_session(login.as_ref(), pass), pg_conn)
            .and_then(|s: Uuid| sessions.find(s).first::<(Uuid, Uuid, String)>(&*pg_conn).map_err(|_| LoadError::NotFound))
            .map(|s| self::Session {
                id: s.0,
                user_id: s.1,
                login: s.2,
                application: app
            })
    }

    pub fn establish<L>(mut app: App, sid: Uuid, ulogin: L) -> Result<Self, LoadError>
        where L: AsRef<str>
    {
        use models::pg::schema::sessions::dsl::*;

        let log = ulogin.as_ref();
        let pg_conn = app.pg().connect();
        sessions.find(sid).filter(login.eq(&log))
            .first::<(Uuid, Uuid, String)>(&*pg_conn)
            .map_err(|_| LoadError::NotFound)
            .map(|s| Session {
                id: s.0,
                user_id: s.1,
                login: s.2,
                application: app,
            })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn admin(mut app: App) -> Result<Self, LoadError> {
        use diesel::associations::HasTable;
        use models::pg::schema::users::dsl::{users, id, login};
        use models::pg::schema::user_group::dsl::{user_group, name};
        use models::pg::schema::user2user_group::dsl::{user2user_group, user_id};
        use models::pg::schema::sessions::dsl::sessions;
        use models::pg::models::*;

        let pg_conn = app.pg().connect();

        let admin_users_id = user_group::table()
            .inner_join(user2user_group::table())
            .filter(name.eq(ADMIN_USER_GROUP))
            .select(user_id);
        let u = users.filter(id.eq_any(admin_users_id)).select((id, login))
            .first::<(Uuid, String)>(&*pg_conn)
            .map_err(|_| LoadError::NotFound)?;

        let s = Session {
            user_id: u.0,
            login: u.1
        };

        ::diesel::insert(&s).into(sessions::table())
            .get_result::<(Uuid, Uuid, String)>(&*pg_conn)
            .map(|e| self::Session {
                id: e.0,
                user_id: e.1,
                login: e.2,
                application: app
            })
            .map_err(|_| LoadError::NotFound)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        sql_function!(delete_session,
                      delete_session_t,
                      (sid: ::diesel::pg::types::sql_types::Uuid) -> Bool);

        let pg_conn = self.application.pg().connect();
        let _res: bool = exec_fn!(delete_session(self.id), pg_conn).unwrap();
    }
}
