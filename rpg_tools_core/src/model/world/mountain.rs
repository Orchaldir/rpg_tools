use crate::utils::storage::{Element, Id};

/// The unique identifier of a [`mountain`](Mountain).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct Mountain {
    id: MountainId,
    name: String,
}

impl Mountain {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Element<MountainId> for Mountain {
    fn new(id: MountainId) -> Self {
        Mountain {
            id,
            name: format!("Mountain {}", id.0),
        }
    }

    fn id(&self) -> MountainId {
        self.id
    }

    fn with_id(self, id: MountainId) -> Self {
        Mountain { id, ..self }
    }

    fn name(&self) -> &str {
        &self.name
    }
}
