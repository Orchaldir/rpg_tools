use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::replace;

pub trait Id: Copy + Hash + Eq + PartialEq {
    fn new(id: usize) -> Self;

    fn id(&self) -> usize;
}

pub trait Element<I: Id>: Eq + PartialEq {
    fn id(&self) -> I;

    fn with_id(self, id: I) -> Self;
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteElementResult<I: Id, T: Element<I>> {
    DeletedLastElement { element: T },
    SwappedAndRemoved { element: T, id_to_update: I },
    NotFound,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Storage<I: Id + PartialEq + Eq, T: Element<I> + PartialEq + Eq> {
    name: String,
    elements: Vec<T>,
    phantom: PhantomData<I>,
}

impl<I: Id, T: Element<I>> Storage<I, T> {
    pub fn new<S: Into<String>>(name: S, elements: Vec<T>) -> Self {
        Self {
            name: name.into(),
            elements,
            phantom: PhantomData,
        }
    }

    pub fn empty<S: Into<String>>(name: S) -> Self {
        Self::new(name, Vec::new())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn create<F: FnOnce(I) -> T>(&mut self, f: F) -> I {
        let id = Id::new(self.elements.len());
        self.elements.push(f(id));
        id
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn contains(&self, id: I) -> bool {
        id.id() < self.elements.len()
    }

    pub fn get_all(&self) -> &Vec<T> {
        &self.elements
    }

    pub fn get(&self, id: I) -> Option<&T> {
        self.elements.get(id.id())
    }

    pub fn get_mut(&mut self, id: I) -> Option<&mut T> {
        self.elements.get_mut(id.id())
    }

    /// Deletes an element by swapping it with the last one, if necessary.
    pub fn delete(&mut self, id: I) -> DeleteElementResult<I, T> {
        let len = self.elements.len();

        if id.id() >= len {
            return DeleteElementResult::NotFound;
        } else if id.id() + 1 == len {
            return DeleteElementResult::DeletedLastElement {
                element: self.elements.pop().unwrap(),
            };
        }

        let last = self.elements.pop().unwrap();

        DeleteElementResult::SwappedAndRemoved {
            element: replace(&mut self.elements[id.id()], last.with_id(id)),
            id_to_update: I::new(len - 1),
        }
    }
}

impl<I: Id, T: Element<I>> Default for Storage<I, T> {
    fn default() -> Self {
        Storage::new("unknown".to_string(), Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::name::EditableName;
    use crate::model::world::town::{Town, TownId};
    use crate::utils::storage::DeleteElementResult::*;

    #[test]
    fn test_create() {
        let mut storage: Storage<TownId, Town> = Storage::default();

        let id = storage.create(Town::new);

        assert_eq!(1, storage.len());
        assert_element(&storage, id, "Town 0");
    }

    #[test]
    fn test_delete_element_in_empty_storage() {
        let mut storage: Storage<TownId, Town> = Storage::default();

        assert_eq!(NotFound, storage.delete(TownId::default()));
    }

    #[test]
    fn test_delete_only_element() {
        let mut storage: Storage<TownId, Town> = Storage::default();
        let id = storage.create(Town::new);

        assert_eq!(
            DeletedLastElement {
                element: Town::new(id),
            },
            storage.delete(id)
        );
        assert!(storage.get_all().is_empty());
    }

    #[test]
    fn test_delete() {
        let mut storage: Storage<TownId, Town> = Storage::default();
        let id0 = storage.create(Town::new);
        let id1 = storage.create(Town::new);
        let id2 = storage.create(Town::new);

        assert_eq!(
            SwappedAndRemoved {
                element: Town::new(id0),
                id_to_update: id2
            },
            storage.delete(id0)
        );

        assert_eq!(2, storage.len());
        assert_element(&storage, id0, "Town 2");
        assert_element(&storage, id1, "Town 1");
    }

    #[test]
    fn test_delete_unknown_index() {
        let mut storage: Storage<TownId, Town> = Storage::default();
        let id = storage.create(Town::new);

        assert_eq!(NotFound, storage.delete(TownId::new(5)));
        assert_eq!(1, storage.len());
        assert_element(&storage, id, "Town 0");
    }

    fn assert_element(storage: &Storage<TownId, Town>, id: TownId, name: &str) {
        let element = storage.get(id).unwrap();

        assert_eq!(id, element.id());
        assert_eq!(name, &element.name().to_string());
    }
}
