
use uuid::Uuid;

use super::schema::*;

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

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User2UserGroup)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub password: String,
    pub email: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub login: &'a str,
    pub password: &'a str,
    pub email: &'a str
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(user2user_group)]
#[table_name="user_group"]
pub struct UserGroup {
    pub id: Uuid,
    pub name: String
}

#[derive(Insertable)]
#[table_name="user_group"]
pub struct NewUserGroup<'a> {
    pub id: Uuid,
    pub name: &'a str
}

#[derive(Insertable, Queryable, Associations)]
#[has_many(users, foreign_key="id")]
#[belongs_to(UserGroup)]
#[table_name="user2user_group"]
pub struct User2UserGroup {
    pub user_id: Uuid,
    pub user_group_id: Uuid
}

#[derive(Insertable, Queryable, Associations)]
#[belongs_to(User)]
#[table_name="sessions"]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid
}
