use crate::utils::storage::{Element, Id};
use serde::{Deserialize, Serialize};

/// The unique identifier of a [`mountain`](Mountain).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MountainId(usize);

impl Id for MountainId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A mountain or hill in the game.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mountain {
    id: MountainId,
    name: String,
}

impl Mountain {
    pub fn new(id: MountainId) -> Self {
        Mountain {
            id,
            name: format!("Mountain {}", id.0),
        }
    }
}

impl Element<MountainId> for Mountain {
    fn id(&self) -> MountainId {
        self.id
    }

    fn with_id(self, id: MountainId) -> Self {
        Mountain { id, ..self }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
