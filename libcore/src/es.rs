
use serde::{Serialize, Deserialize};
use chrono::naive::datetime::NaiveDateTime;
use uuid::Uuid;

use rs_es::error;
use rs_es::Client;
use rs_es::query::*;
use rs_es::operations::get::GetResult;
use rs_es::operations::index::IndexResult;
use rs_es::operations::delete::DeleteResult;
use rs_es::operations::search::{SearchHitsResult, SearchResult};

use FromDto;
use connection::App;

pub trait EsDto: Serialize + Deserialize {
    fn doc_type() -> &'static str;
    fn id(&self) -> Uuid;
}

struct EsClient {
    index: String,
    client: Client,
}

impl EsClient {
    #[inline]
    fn new(url: String, index: String) -> Result<EsClient, EsError> {
        Ok(EsClient {
               index: index,
               client: Client::new(&url).map_err(|_| EsError::InvalidUrl)?,
           })
    }

    fn index<'a, 'b, T: EsDto>(&'a mut self, doc: &'b T) -> Result<IndexResult, error::EsError> {
        self.client
            .index(&self.index, T::doc_type())
            .with_doc(doc)
            .with_id(doc.id().hyphenated().to_string().as_str())
            .send()
    }

    fn delete<'a, 'b, T: EsDto>(&'a mut self, id: Uuid) -> Result<DeleteResult, error::EsError> {
        self.client
            .delete(&self.index,
                    T::doc_type(),
                    id.hyphenated().to_string().as_str())
            .send()
    }

    fn get<'a, 'b, T: EsDto>(&'a mut self, id: Uuid) -> Result<GetResult<T>, error::EsError> {
        self.client
            .get(&self.index, id.hyphenated().to_string().as_str())
            .with_doc_type(T::doc_type())
            .send()
    }

    fn search<'a, 'b, T: EsDto>(&'a mut self,
                                q: &'b Query)
                                -> Result<SearchResult<T>, error::EsError> {
        self.client
            .search_query()
            .with_indexes(&[&self.index])
            .with_types(&[T::doc_type()])
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

struct EsRepository {
    client: EsClient,
}

impl EsRepository {
    fn new(url: String, index: String) -> EsRepository {
        EsRepository {
            client: EsClient::new(url, index).expect("Unable to connect to elasticsearch"),
        }
    }

    fn get<T: EsDto>(&mut self, id: Uuid) -> Result<T, EsError> {
        match self.client.get::<T>(id) {
            Ok(GetResult { source: Some(doc), .. }) => Ok(doc),
            _ => Err(EsError::NotFound),
        }
    }

    fn search<T: EsDto>(&mut self, query: Query) -> Result<Vec<Box<T>>, EsError> {
        match self.client.search::<T>(&query) {
            Ok(SearchResult { hits: SearchHitsResult { hits: mut result, .. }, .. }) => {
                let docs = result
                    .drain(..)
                    .map(|h| h.source.and_then(|x| Some(x)))
                    .filter(|h| h.is_some())
                    .map(|h| h.unwrap())
                    .collect::<Vec<Box<T>>>();
                Ok(docs)
            }
            _ => Err(EsError::NotFound),
        }
    }

    fn index<T: EsDto>(&mut self, item: &T) -> Result<(), EsError> {
        match self.client.index(item) {
            Ok(IndexResult { created, .. }) if created => Ok(()),
            Ok(IndexResult { created, .. }) if !created => Err(EsError::CreationFailed),
            Err(inner) => Err(EsError::Inner(inner)),
            Ok(_) => unreachable!(),
        }
    }

    fn delete<T: EsDto>(&mut self, id: Uuid) -> Result<(), EsError> {
        match self.client.delete::<T>(id) {
            Ok(DeleteResult { found, .. }) if found => Ok(()),
            Ok(DeleteResult { found, .. }) if !found => Err(EsError::NotFound),
            Err(inner) => Err(EsError::Inner(inner)),
            Ok(_) => unreachable!(),
        }
    }
}

pub struct EsService {
    client: EsRepository,
}

impl EsService {
    pub fn new(url: String, index: String) -> EsService {
        EsService { client: EsRepository::new(url, index) }
    }

    pub fn by_id<D: EsDto, T: FromDto<Dto = D>>(&mut self,
                                                app: App,
                                                id: Uuid)
                                                -> Result<T, EsError> {
        self.client
            .get::<D>(id)
            .map_err(|_| EsError::NotFound)
            .and_then(|d| Ok(T::from_dto(d, app)))
    }

    pub fn index<T: EsDto>(&mut self, item: &T) -> Result<(), EsError> {
        self.client.index(item)
    }

    pub fn delete<T: EsDto>(&mut self, id: Uuid) -> Result<(), EsError> {
        self.client.delete::<T>(id)
    }
}
