use crate::model::world::street::StreetId;

/// An edge of the [`town`](crate::model::world::town::Town) map.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TownEdge {
    None,
    Street { id: StreetId },
}

impl TownEdge {
    pub fn street(id: StreetId) -> Self {
        Self::Street { id }
    }
}
