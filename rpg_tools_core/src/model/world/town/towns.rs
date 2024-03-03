use crate::model::world::town::TownId;
use std::collections::HashSet;

pub trait WithTowns {
    fn towns(&self) -> &HashSet<TownId>;
    fn towns_mut(&mut self) -> &mut HashSet<TownId>;
}
