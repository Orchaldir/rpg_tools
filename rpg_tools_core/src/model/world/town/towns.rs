use crate::model::world::town::TownId;
use std::collections::HashSet;

/// A trait for objects connected to multiple [`towns`](crate::model::world::town::Town).
pub trait WithTowns {
    fn towns(&self) -> &HashSet<TownId>;
    fn towns_mut(&mut self) -> &mut HashSet<TownId>;
}
