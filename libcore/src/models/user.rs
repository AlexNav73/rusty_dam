
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;
use crypto::Keccak;

use {Load, LoadError};
use connection::App;

pub struct User<'a> {
    id: Uuid,
    login: String,
    password: String,
    email: Option<String>,
    is_new: bool,
    is_dirty: (bool, bool, bool),
    application: App<'a>,
}

impl<'a> User<'a> {
    pub fn new<L, P, E>(app: App, login: L, password: P, email: Option<E>) -> Result<Self, LoadError>
        where L: Into<String>,
              P: Into<String>,
              E: Into<String>
    {
        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        Ok(User {
               id: Uuid::new_v4(),
               login: login.into(),
               password: password.into(),
               email: email.map(|e| e.into()),
               is_new: true,
               is_dirty: (false, false, false),
               application: app,
           })
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty.0 || self.is_dirty.1 || self.is_dirty.2
    }

    pub fn save(&mut self) -> Result<(), LoadError> {
        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        if self.is_new {
            self.save_new()
        } else if self.is_dirty() {
            self.update()
        } else {
            Ok(())
        }
    }

    fn update(&mut self) -> Result<(), LoadError> {
        use models::pg::schema::users::dsl::*;
        use models::pg::models::*;

        let mut changes = UserChangeset::default();
        if self.is_dirty.0 {
            changes.login = Some(&self.login);
            self.is_dirty.0 = false;
        }
        if self.is_dirty.1 {
            changes.password = Some(&self.password);
            self.is_dirty.1 = false;
        }
        if self.is_dirty.2 {
            changes.email = Some(self.email.as_ref().map(|s| s.as_str()));
            self.is_dirty.2 = false;
        }

        let pg_conn = self.application.pg().connect();
        ::diesel::update(users.find(self.id))
            .set(&changes)
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

    fn save_new(&mut self) -> Result<(), LoadError> {
        use diesel::associations::HasTable;
        use models::pg::models::*;
        use models::pg::schema::users::dsl::*;

        self.is_new = false;
        let new_user = NewUser {
            id: self.id,
            login: &self.login,
            password: &get_sha3(self.password.as_str()),
            email: self.email.as_ref().map(|s| s.as_str()),
        };

        let pg_conn = self.application.pg().connect();
        ::diesel::insert(&new_user)
            .into(users::table())
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

    pub fn delete(mut self) -> Result<(), LoadError> {
        use models::pg::schema::users::dsl::*;

        if self.application.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = self.application.pg().connect();
        ::diesel::delete(users.find(self.id))
            .execute(&*pg_conn)
            .map(|_| ())
            .map_err(|_| LoadError::NotFound)
    }

    pub unsafe fn create_administrator<L, P>(mut app: App, login: L, password: P) -> Result<Uuid, UserError>
        where L: AsRef<str>, P: AsRef<str>
    {
        use diesel::types::Text;

        sql_function!(create_admin,
                      create_admin_t,
                      (uname: Text, passwd: Text) -> ::diesel::pg::types::sql_types::Uuid);

        let pg_conn = app.pg().connect();
        ::diesel::select(create_admin(login.as_ref(), get_sha3(password))).first::<Uuid>(&*pg_conn)
            .map_err(|e| match e {
                Error::NotFound => UserError::NotFound,
                Error::DatabaseError(_, er) => UserError::DatabaseError(er.message().to_owned()),
                _ => UserError::UnexpectedError
            })
    }
}

fn get_sha3<S: AsRef<str>>(text: S) -> String {
        let mut sha3 = Keccak::new_sha3_256();
        sha3.update(text.as_ref().as_bytes());
        let mut res = [0; 32];
        sha3.finalize(&mut res);

        String::from_utf8_lossy(&res).into_owned()
}

impl<'a> Load for User<'a> {
    fn load(mut app: App, uid: Uuid) -> Result<Self, LoadError> {
        use models::pg::schema::users::dsl::*;
        use models::pg::models::*;

        if app.session().is_none() {
            return Err(LoadError::Unauthorized);
        }

        let pg_conn = app.pg().connect();
        users
            .find(uid)
            .first::<User>(&*pg_conn)
            .map_err(|_| LoadError::NotFound)
            .and_then(|u| {
                Ok(self::User {
                       id: u.id,
                       login: u.login,
                       password: u.password,
                       email: u.email,
                       is_new: false,
                       is_dirty: (false, false, false),
                       application: app,
                   })
            })
    }
}

#[derive(Debug)]
pub enum UserError {
    NotFound,
    UnexpectedError,
    DatabaseError(String)
}
