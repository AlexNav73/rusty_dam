
use uuid::Uuid;

use super::schema::*;

#[derive(Debug, Queryable)]
pub struct Classification {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
}

#[derive(Default, AsChangeset)]
#[table_name="classifications"]
pub struct ClassificationChangeset {
    pub parent_id: Option<Option<Uuid>>,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name="classifications"]
pub struct NewClassification<'a> {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: &'a str,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[has_many(field2field_groups)]
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
    pub name: String,
}

#[derive(Insertable, Queryable, Associations)]
#[belongs_to(Field)]
#[belongs_to(FieldGroup)]
#[table_name="field2field_groups"]
pub struct Field2FieldGroup {
    pub field_id: Uuid,
    pub field_group_id: Uuid,
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(user2user_group)]
#[has_many(sessions)]
#[table_name="users"]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Default, AsChangeset)]
#[table_name="users"]
pub struct UserChangeset<'a> {
    pub login: Option<&'a str>,
    pub password: Option<&'a str>,
    pub email: Option<Option<&'a str>>,
}

impl<'a> UserChangeset<'a> {
    pub fn is_dirty(&self) -> bool {
        self.login.is_some() || self.password.is_some() || self.email.is_some()
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub login: &'a str,
    pub password: &'a str,
    pub email: Option<&'a str>,
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(user2user_group)]
#[table_name="user_group"]
pub struct UserGroup {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="user_group"]
pub struct NewUserGroup<'a> {
    pub id: Uuid,
    pub name: &'a str,
}

#[derive(Insertable, Queryable, Associations)]
#[belongs_to(User)]
#[belongs_to(UserGroup)]
#[table_name="user2user_group"]
pub struct User2UserGroup {
    pub user_id: Uuid,
    pub user_group_id: Uuid,
}

#[derive(Insertable, Queryable, Associations)]
#[belongs_to(User, foreign_key="user_id")]
#[table_name="sessions"]
pub struct Session {
    pub user_id: Uuid,
    pub login: String,
}
