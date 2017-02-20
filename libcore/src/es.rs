
use chrono::naive::datetime::NaiveDateTime;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

use rs_es::Client;
use rs_es::operations::get::{ GetOperation, GetResult };
use rs_es::operations::index::IndexOperation;
use rs_es::operations::search::{ SearchQueryOperation, SearchHitsResult, SearchHitsHitsResult, SearchResult };
use rs_es::query::*;

use std::mem;
use std::fmt;

use { Entity, Document };

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

    pub fn index<'a, 'b, T: Entity>(&'a mut self, doc: &'b T) -> &'a mut IndexOperation<'a, 'b, T::Dto> {
        // FIXME: Remove mem::transmute when rs-es fix operations lifetime rules
        unsafe {
            mem::transmute(self.client
                .index(ES_INDEX_NAME, T::Dto::doc_type())
                .with_doc(&doc.map())
                .with_id(doc.id().hyphenated().to_string().as_str()))
        }
    }

    pub fn get<'a, 'b, T: Entity>(&'a mut self, id: Uuid) -> &'a mut GetOperation<'a, 'b> {
        unsafe {
            mem::transmute(self.client
                           .get(ES_INDEX_NAME, id.hyphenated().to_string().as_str())
                           .with_doc_type(T::Dto::doc_type()))
        }
    }

    pub fn search<'a, 'b, T: Entity>(&'a mut self, q: &'b Query) -> &'a mut SearchQueryOperation<'a, 'b> {
        unsafe {
            mem::transmute(self.client
                           .search_query()
                           .with_indexes(&[ES_INDEX_NAME])
                           .with_types(&[T::Dto::doc_type()])
                           .with_query(q))
        }
    }
}

pub enum EsError {
    InvalidUrl,
    NotFound
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
    client: EsClient
}

impl EsRepository {
    pub fn new<S: AsRef<str>>(url: S) -> EsRepository {
        EsRepository { 
            client: EsClient::new(url.as_ref())
                .expect("Unable to connect to elasticsearch")
        }
    }

    pub fn get<T: Entity>(&mut self, id: Uuid) -> Result<T, EsError> {
        match self.client.get::<T>(id).send() {
            Ok(GetResult { source: Some(doc), .. }) => {
                let doc: T::Dto = doc;
                Ok(doc.map())
            },
            _ => Err(EsError::NotFound)
        }
    }

    pub fn search<T: Entity>(&mut self, query: Query) -> Result<Vec<Box<T>>, EsError> {
        match self.client.search::<T>(&query).send() {
            Ok(SearchResult { hits: SearchHitsResult { hits: mut result , .. }, .. }) => {
                let docs = result
                    .drain(..)
                    .map(|h: SearchHitsHitsResult<T::Dto>| h.source.and_then(|x| Some(x)))
                    .filter(|h| h.is_some())
                    .map(|h| Box::new(h.unwrap().map()))
                    .collect::<Vec<Box<T>>>();
                Ok(docs)
            },
            _ => Err(EsError::NotFound)
        }
    }

    pub fn index<T: Entity>(&mut self, item: &T) {
        self.client.index(item).send();
    }
}

