
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct Classification {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
}
