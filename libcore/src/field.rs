
use Entity;

use uuid::Uuid;

pub struct Field {
    id: Uuid
}

impl Entity for Field {
    fn id(&self) -> Uuid { self.id }
}

