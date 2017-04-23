
use uuid::Uuid;

use super::schema::classifications;
use super::schema::fields;

#[derive(Debug, Queryable)]
pub struct Classification {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="classifications"]
pub struct NewClassification<'a> {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: &'a str,
}

#[derive(Debug, Queryable)]
pub struct Fields {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="fields"]
pub struct NewField<'a> {
    pub id: Uuid,
    pub name: &'a str,
}
