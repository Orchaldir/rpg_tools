use crate::model::world::mountain::MountainId;
use crate::model::world::river::RiverId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Terrain {
    Hill { id: MountainId },
    Mountain { id: MountainId },
    Plain,
    River { id: RiverId },
}
