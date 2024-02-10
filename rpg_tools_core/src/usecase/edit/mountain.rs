use crate::model::world::mountain::MountainId;
use crate::model::world::WorldData;
use crate::utils::storage::Element;
use anyhow::{bail, Context, Result};

/// Tries to update the name of a [`mountain`](crate::model::mountain::Mountain).
pub fn update_mountain_name(data: &mut WorldData, id: MountainId, name: &str) -> Result<()> {
    let trimmed = name.trim().to_string();

    if trimmed.is_empty() {
        bail!("Name is empty!")
    } else if data
        .mountain_manager
        .get_all()
        .iter()
        .filter(|r| r.id().ne(&id))
        .any(|r| r.name().eq(&trimmed))
    {
        bail!("Name '{}' already exists!", trimmed)
    }

    data.mountain_manager
        .get_mut(id)
        .map(|r| r.set_name(trimmed))
        .context("Mountain doesn't exist")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::storage::Id;

    #[test]
    fn update_empty_name() {
        let mut data = WorldData::default();

        assert!(update_mountain_name(&mut data, MountainId::new(0), "").is_err());
    }

    #[test]
    fn update_name_contains_only_whitespaces() {
        let mut data = WorldData::default();

        assert!(update_mountain_name(&mut data, MountainId::new(0), "  ").is_err());
    }

    #[test]
    fn update_name_of_non_existing_mountain() {
        let mut data = WorldData::default();

        assert!(update_mountain_name(&mut data, MountainId::new(0), "Test").is_err());
    }

    #[test]
    fn update_valid_name() {
        test_update_name("Test", "Test");
    }

    #[test]
    fn update_trimmed_name() {
        test_update_name(" Name ", "Name");
    }

    #[test]
    fn update_duplicate_name() {
        let mut data = WorldData::default();
        let id0 = data.mountain_manager.create();
        let id1 = data.mountain_manager.create();

        assert!(update_mountain_name(&mut data, id0, "Test").is_ok());
        assert!(update_mountain_name(&mut data, id1, "Test").is_err());
    }

    fn test_update_name(input: &str, result: &str) {
        let mut data = WorldData::default();
        let id = data.mountain_manager.create();

        assert!(update_mountain_name(&mut data, id, input).is_ok());

        assert_eq!(
            result,
            data.mountain_manager.get(id).map(|r| r.name()).unwrap()
        );
    }
}
