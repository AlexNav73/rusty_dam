
use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::ops::Deref;

pub trait PgDto {}

struct PgClient {
    url: String,
}

#[macro_export]
macro_rules! exec_fn {
    ($call:expr, $connection:expr) => {{
        ::diesel::select($call).first(&*$connection).map_err(|_| LoadError::NotFound)
    }}
}

impl PgClient {
    fn new(url: String) -> PgClient {
        PgClient { url: url }
    }

    fn connect(&self) -> PgClientConnection {
        PgClientConnection::new(&self.url)
    }
}

pub struct PgClientConnection {
    client: PgConnection,
}

impl PgClientConnection {
    fn new(url: &str) -> Self {
        PgClientConnection {
            client: PgConnection::establish(&url).expect(&format!("Error connecting to {}", url)),
        }
    }
}

impl Deref for PgClientConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

pub struct PgService {
    client: PgClient,
}

impl PgService {
    pub fn new(url: String) -> PgService {
        PgService { client: PgClient::new(url) }
    }

    pub fn connect(&self) -> PgClientConnection {
        self.client.connect()
    }
}



//note: the method `first` exists but the following trait bounds were not satisfied:
//`_ : diesel::query_builder::QueryFragment<_>`,
//`_ : diesel::query_builder::QueryId`,
//`_ : diesel::query_builder::QueryFragment<_>`,
//`_ : diesel::query_builder::QueryId`,
//`&mut diesel::query_builder::SelectStatement<(),
//diesel::query_builder::select_clause::SelectClause<F>> : diesel::query_builder::AsQuery`,
//`_ : diesel::query_builder::QueryFragment<_>`,
//`_ : diesel::query_builder::QueryId`,
//`&mut diesel::query_builder::SelectStatement<(),
//diesel::query_builder::select_clause::SelectClause<F>> : diesel::query_builder::Query`,
//`&mut diesel::query_builder::SelectStatement<(),
//diesel::query_builder::select_clause::SelectClause<F>> : diesel::query_builder::Query`,
//`&mut diesel::query_builder::SelectStatement<(),
//diesel::query_builder::select_clause::SelectClause<F>> : diesel::query_builder::Query`
