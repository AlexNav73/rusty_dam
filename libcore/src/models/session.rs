
use diesel::prelude::*;
use diesel::types::*;
use uuid::Uuid;

use LoadError;
use connection::App;

pub struct Session {
    id: Uuid,
    user_id: Option<Uuid>,
    login: Option<String>,
    application: App,
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
            .and_then(|s| Ok(Session { id: s, user_id: None, login: None, application: app }))
    }

    pub fn login(&mut self) -> &str {
        use models::pg::schema::sessions::dsl::*;
        use models::pg::models::Session;

        if self.user_id.is_none() || self.login.is_none() {
            let pg_conn = self.application.pg().connect();

            let s = sessions
                .filter(id.eq(self.id))
                .first::<Session>(&*pg_conn)
                .unwrap();
            self.user_id = Some(s.user_id);
            self.login = Some(s.login);
        }

        if let Some(ref s) = self.login {
            s
        } else {
            unreachable!()
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        sql_function!(delete_session,
                      delete_session_t,
                      (sid: ::diesel::pg::types::sql_types::Uuid) -> Bool);

        let pg_conn = self.application.pg().connect();
        let res: bool = exec_fn!(delete_session(self.id), pg_conn).unwrap();
        if !res {
            panic!("Session with id: {} does not exists", self.id)
        }
    }
}
