
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct Classification {
    id: Uuid,
    parent_id: Option<Uuid>,
    name: String,
}
