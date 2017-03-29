
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

use {Entity, Document};
use connection::Connection;

pub struct EsClient {
    index: String,
    client: Client,
}

impl EsClient {
    #[inline]
    pub fn new(url: String, index: String) -> Result<EsClient, EsError> {
        Ok(EsClient {
               index: index,
               client: Client::new(&url).map_err(|_| EsError::InvalidUrl)?,
           })
    }

    pub fn index<'a, 'b, T: Entity>(&'a mut self,
                                    doc: &'b T)
                                    -> Result<IndexResult, error::EsError> {
        self.client
            .index(&self.index, T::Dto::doc_type())
            .with_doc(&doc.map())
            .with_id(doc.id().hyphenated().to_string().as_str())
            .send()
    }

    pub fn get<'a, 'b, T: Entity>(&'a mut self,
                                  id: Uuid)
                                  -> Result<GetResult<T::Dto>, error::EsError> {
        self.client
            .get(&self.index, id.hyphenated().to_string().as_str())
            .with_doc_type(T::Dto::doc_type())
            .send()
    }

    pub fn search<'a, 'b, T: Entity>(&'a mut self,
                                     q: &'b Query)
                                     -> Result<SearchResult<T::Dto>, error::EsError> {
        self.client
            .search_query()
            .with_indexes(&[&self.index])
            .with_types(&[T::Dto::doc_type()])
            .with_query(q)
            .send()
    }
}

#[derive(Debug)]
pub enum EsError {
    InvalidUrl,
    NotFound,
    CreationFailed,
    Inner(error::EsError),
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
    pub fn new(url: String, index: String) -> EsRepository {
        EsRepository {
            client: EsClient::new(url, index).expect("Unable to connect to elasticsearch"),
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
                let docs = result
                    .drain(..)
                    .map(|h: SearchHitsHitsResult<T::Dto>| h.source.and_then(|x| Some(x)))
                    .filter(|h| h.is_some())
                    .map(|h| Box::new(h.unwrap().map(conn.clone())))
                    .collect::<Vec<Box<T>>>();
                Ok(docs)
            }
            _ => Err(EsError::NotFound),
        }
    }

    pub fn index<T: Entity>(&mut self, item: &T) -> Result<(), EsError> {
        match self.client.index(item) {
            Ok(IndexResult { created, .. }) if created => Ok(()),
            Ok(IndexResult { created, .. }) if !created => Err(EsError::CreationFailed),
            Err(inner) => Err(EsError::Inner(inner)),
            Ok(_) => unreachable!(),
        }
    }
}
