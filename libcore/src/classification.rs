
use uuid::Uuid;

use Entity;

pub struct Classification {
    id: Uuid
}

impl Entity for Classification {
    fn id(&self) -> Uuid { self.id }
}

