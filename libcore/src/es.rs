
use chrono::naive::datetime::NaiveDateTime;
use uuid::Uuid;

use std::rc::Rc;
use std::cell::RefCell;

use rs_es::error;
use rs_es::Client;
use rs_es::query::*;
use rs_es::operations::get::GetResult;
use rs_es::operations::index::IndexResult;
use rs_es::operations::search::{SearchHitsResult, SearchHitsHitsResult, SearchResult};

use std::fmt;

use {Entity, Document};
use connection::Connection;

// TODO: Index must be named be connection name to allow
//       multiple indices in one cluster. Sould be taken from config
const ES_INDEX_NAME: &'static str = "rusty_dam";

pub struct EsClient {
    client: Client,
}

impl EsClient {
    #[inline]
    pub fn new<S: AsRef<str>>(url: S) -> Result<EsClient, EsError> {
        Ok(EsClient { client: Client::new(url.as_ref()).map_err(|_| EsError::InvalidUrl)? })
    }

    pub fn index<'a, 'b, T: Entity>(&'a mut self,
                                    doc: &'b T)
                                    -> Result<IndexResult, error::EsError> {
        self.client
            .index(ES_INDEX_NAME, T::Dto::doc_type())
            .with_doc(&doc.map())
            .with_id(doc.id().hyphenated().to_string().as_str())
            .send()
    }

    pub fn get<'a, 'b, T: Entity>(&'a mut self,
                                  id: Uuid)
                                  -> Result<GetResult<T::Dto>, error::EsError> {
        self.client
            .get(ES_INDEX_NAME, id.hyphenated().to_string().as_str())
            .with_doc_type(T::Dto::doc_type())
            .send()
    }

    pub fn search<'a, 'b, T: Entity>(&'a mut self,
                                     q: &'b Query)
                                     -> Result<SearchResult<T::Dto>, error::EsError> {
        self.client
            .search_query()
            .with_indexes(&[ES_INDEX_NAME])
            .with_types(&[T::Dto::doc_type()])
            .with_query(q)
            .send()
    }
}

pub enum EsError {
    InvalidUrl,
    NotFound,
}

impl fmt::Debug for EsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to connect to elasticsearch using this adress")
    }
}

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub id: Uuid,
    pub created_by: String,
    pub created_on: NaiveDateTime,
    pub modified_by: String,
    pub modified_on: NaiveDateTime,
}

pub struct EsRepository {
    client: EsClient,
}

impl EsRepository {
    pub fn new<S: AsRef<str>>(url: S) -> EsRepository {
        EsRepository {
            client: EsClient::new(url.as_ref()).expect("Unable to connect to elasticsearch"),
        }
    }

    pub fn get<T: Entity>(&mut self,
                          conn: Rc<RefCell<Connection>>,
                          id: Uuid)
                          -> Result<T, EsError> {
        match self.client.get::<T>(id) {
            Ok(GetResult { source: Some(doc), .. }) => {
                let doc: T::Dto = doc;
                Ok(doc.map(conn))
            }
            _ => Err(EsError::NotFound),
        }
    }

    pub fn search<T: Entity>(&mut self,
                             conn: Rc<RefCell<Connection>>,
                             query: Query)
                             -> Result<Vec<Box<T>>, EsError> {
        match self.client.search::<T>(&query) {
            Ok(SearchResult { hits: SearchHitsResult { hits: mut result, .. }, .. }) => {
                let docs = result.drain(..)
                    .map(|h: SearchHitsHitsResult<T::Dto>| h.source.and_then(|x| Some(x)))
                    .filter(|h| h.is_some())
                    .map(|h| Box::new(h.unwrap().map(conn.clone())))
                    .collect::<Vec<Box<T>>>();
                Ok(docs)
            }
            _ => Err(EsError::NotFound),
        }
    }

    pub fn index<T: Entity>(&mut self, item: &T) {
        let _ = self.client.index(item);
    }
}
