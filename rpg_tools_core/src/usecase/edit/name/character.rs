use crate::model::character::name::CharacterName;
use crate::model::character::CharacterId;
use crate::model::RpgData;
use crate::utils::storage::Element;
use anyhow::{bail, Context, Result};

/// Tries to update the name of a [`character`](crate::model::character::Character).
pub fn update_character_name(
    data: &mut RpgData,
    id: CharacterId,
    name: CharacterName,
) -> Result<()> {
    if data
        .characters
        .get_all()
        .iter()
        .filter(|r| r.id().ne(&id))
        .any(|r| r.name.eq(&name))
    {
        bail!("Name '{}' already exists!", name)
    }

    data.characters
        .get_mut(id)
        .map(|r| r.name = name)
        .context("Id doesn't exist")
}
