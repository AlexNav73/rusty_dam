
use uuid::Uuid;
use rocket_contrib::UUID;
use rocket_contrib::JSON;

use libcore::Classification;

use APIKey;

#[derive(Serialize)]
struct ClassificationTreeNode<'a> {
    id: Uuid,
    children: bool,
    text: String,
    icon: &'a str
}

impl<'a> ClassificationTreeNode<'a> {
    fn new(id: Uuid, text: String, children: bool) -> Self {
        ClassificationTreeNode { id, text, children, icon: "/" }
    }
}

#[derive(FromForm)]
struct NodeId {
    id: UUID
}

#[get("/parents?<cls>")]
fn get_root_classifications<'a>(app: APIKey, cls: NodeId) -> JSON<ClassificationTreeNode<'a>> {
    let root = app.0.get::<Classification>(*cls.id).unwrap();
    JSON(ClassificationTreeNode::new(root.id().clone(), root.name().into(), true))
}

#[allow(unused_variables)]
#[get("/children?<cls>")]
fn get_child_classifications<'a>(_app: APIKey, cls: NodeId) -> JSON<ClassificationTreeNode<'a>> {
    JSON(ClassificationTreeNode::new(Uuid::new_v4(), "Child".into(), true))
}

