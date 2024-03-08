use crate::model::character::gender::Gender;
use crate::model::character::CharacterId;
use crate::model::RpgData;
use anyhow::{Context, Result};

/// Tries to update the [`gender`](Gender) of an [`character`](crate::model::character::Character).
pub fn update_gender(data: &mut RpgData, id: CharacterId, gender: Gender) -> Result<()> {
    data.characters
        .get_mut(id)
        .map(|character| character.gender = gender)
        .context("Character doesn't exist")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::Character;
    use crate::model::RpgData;

    #[test]
    fn success() {
        let mut data = RpgData::default();
        let id = data.characters.create(Character::new);

        assert!(update_gender(&mut data, id, Gender::Genderless).is_ok());

        assert_eq!(data.characters.get(id).unwrap().gender, Gender::Genderless);
    }

    #[test]
    fn non_existing_character() {
        let mut data = RpgData::default();

        assert!(update_gender(&mut data, CharacterId::default(), Gender::Female).is_err());

        assert!(data.characters.is_empty());
    }
}
