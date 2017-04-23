
use uuid::Uuid;

use super::schema::classifications;
use super::schema::fields;
use super::schema::field_groups;
use super::schema::field2field_groups;

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

#[derive(Debug, Identifiable, Queryable, Associations)]
#[belongs_to(Field2FieldGroup)]
pub struct Field {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="fields"]
pub struct NewField<'a> {
    pub id: Uuid,
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(field2field_groups)]
pub struct FieldGroup {
    pub id: Uuid,
    pub name: String
}

#[derive(Insertable, Queryable, Associations)]
#[has_many(fields, foreign_key="id")]
#[belongs_to(FieldGroup)]
#[table_name="field2field_groups"]
pub struct Field2FieldGroup {
    pub field_id: Uuid,
    pub field_group_id: Uuid
}
