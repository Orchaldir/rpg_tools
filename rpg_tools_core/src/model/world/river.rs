use crate::utils::storage::{Element, Id};

/// The unique identifier of a [`river`](River).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct RiverId(usize);

impl Id for RiverId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A river in the game.
#[derive(Clone, Debug, PartialEq)]
pub struct River {
    id: RiverId,
    name: String,
}

impl River {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Element<RiverId> for River {
    fn new(id: RiverId) -> Self {
        River {
            id,
            name: format!("River {}", id.0),
        }
    }

    fn id(&self) -> RiverId {
        self.id
    }

    fn with_id(self, id: RiverId) -> Self {
        River { id, ..self }
    }
}