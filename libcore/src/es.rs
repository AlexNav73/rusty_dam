
use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use rs_es::Client;
use rs_es::operations::index::IndexOperation;
use chrono::naive::datetime::NaiveDateTime;

use std::mem;

use ::Entity;

// TODO: Index must be named be connection name to allow 
//       multiple indices in one cluster. Sould be taken from config
const ES_INDEX_NAME: &'static str = "rusty_dam";

///
/// All documents which needs to be stored in elasticsearch must
/// implement this trait.
///
pub trait EsDocument: Serialize + Deserialize {
    ///
    /// Document type used by elasticsearch to distinguish documents
    ///
    fn entity_type() -> &'static str;
}

struct EsClient {
    client: Client
}

impl EsClient {
    #[inline]
    pub fn new<S: AsRef<str>>(url: S) -> Result<EsClient, EsClientError> {
        Ok(EsClient{
            client: Client::new(url.as_ref()).map_err(|_| EsClientError::InvalidUrl)?
        })
    }

    pub fn index<'a, 'b, T>(&'a mut self, doc: &'b T) -> &'a mut IndexOperation<'a, 'b, T> 
        where T: EsDocument + Entity  
    {
        // FIXME: Remove mem::transmute when rs-es fix operations lifetime rules
        unsafe {
            mem::transmute(self.client.index(ES_INDEX_NAME, T::entity_type())
                                      .with_doc(doc)
                                      .with_id(doc.id().hyphenated().to_string().as_str()))
        }
    }
}

pub enum EsClientError {
    InvalidUrl
}

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub id: Uuid,
    pub created_by: String,
    pub created_on: NaiveDateTime,
    pub modified_by: String,
    pub modified_on: NaiveDateTime
}

