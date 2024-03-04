use crate::model::world::mountain::MountainId;
use crate::model::world::river::RiverId;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Terrain {
    Hill { id: MountainId },
    Mountain { id: MountainId },
    Plain,
    River { id: RiverId },
}
