
use uuid::Uuid;
use rs_es::Client;
use rs_es::operations::index::IndexOperation;
use rs_es::operations::get::GetOperation;
use chrono::naive::datetime::NaiveDateTime;

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
    pub fn new<S: AsRef<str>>(url: S) -> Result<EsClient, EsClientError> {
        Ok(EsClient { client: Client::new(url.as_ref()).map_err(|_| EsClientError::InvalidUrl)? })
    }

    pub fn index<'a, 'b, T, U>(&'a mut self, doc: &'b T) -> &'a mut IndexOperation<'a, 'b, T>
        where T: Entity + Document<U>,
              U: Entity
    {
        // FIXME: Remove mem::transmute when rs-es fix operations lifetime rules
        unsafe {
            mem::transmute(self.client
                .index(ES_INDEX_NAME, T::doc_type())
                .with_doc(doc)
                .with_id(doc.id().hyphenated().to_string().as_str()))
        }
    }

    pub fn find_by_id<'a, 'b, U: Entity, T: Document<U>>(&'a mut self, id: Uuid) -> &'a mut GetOperation<'a, 'b> {
        unsafe {
            mem::transmute(self.client.get(ES_INDEX_NAME, id.hyphenated().to_string().as_str())
                .with_doc_type(T::doc_type()))
        }
    }
}

pub enum EsClientError {
    InvalidUrl,
}

impl fmt::Debug for EsClientError {
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

