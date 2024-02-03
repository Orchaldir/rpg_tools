use crate::model::world::mountain::MountainId;
use crate::model::world::river::RiverId;

pub enum Terrain {
    Hill { id: MountainId },
    Mountain { id: MountainId },
    Plain,
    River { id: RiverId },
}
