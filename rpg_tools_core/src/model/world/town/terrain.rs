use crate::model::world::mountain::MountainId;
use crate::model::world::river::RiverId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Terrain {
    Hill { id: MountainId },
    Mountain { id: MountainId },
    Plain,
    River { id: RiverId },
}
