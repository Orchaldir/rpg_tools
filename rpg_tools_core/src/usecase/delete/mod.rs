pub mod building;

use crate::model::world::town::TownId;

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteResult {
    Ok,
    NotFound,
    Blocked(BlockingReason),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BlockingReason {
    pub towns: Vec<TownId>,
}
