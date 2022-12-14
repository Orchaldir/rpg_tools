use crate::model::character::{Character, CharacterId};
use anyhow::Result;

/// Stores all the [`Character`]s.
#[derive(Default, Debug)]
pub struct CharacterMgr {
    characters: Vec<Character>,
}

impl CharacterMgr {
    /// Uses the function *f* to create a [`Character`] with the next [`CharacterId`].
    pub fn create<F>(&mut self, f: F) -> Result<CharacterId>
    where
        F: FnOnce(CharacterId) -> Result<Character>,
    {
        let id = CharacterId::new(self.characters.len());
        self.characters.push(f(id)?);
        Ok(id)
    }

    pub fn get_all(&self) -> &Vec<Character> {
        &self.characters
    }

    pub fn get(&self, id: CharacterId) -> Option<&Character> {
        self.characters.get(id.id())
    }

    pub fn get_mut(&mut self, id: CharacterId) -> Option<&mut Character> {
        self.characters.get_mut(id.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::gender::Gender;
    use Gender::*;

    #[test]
    fn test_create() {
        let mut manager = CharacterMgr::default();

        let id0 = manager
            .create(|id| Ok(Character::simple(id, 0, Female)))
            .unwrap();
        let id1 = manager
            .create(|id| Ok(Character::simple(id, 1, Male)))
            .unwrap();

        assert_ne!(id0, id1);
        assert_eq!(id0, manager.get(id0).unwrap().id());
        assert_eq!(id1, manager.get(id1).unwrap().id());
    }
}
