use crate::model::name::{EditableName, Name};
use crate::utils::storage::{Element, Id, Storage};
use anyhow::{bail, Context, Result};

/// Tries to update the name of an [`element`](Element).
pub fn update_name<ID: Id, ELEMENT: Element<ID> + EditableName>(
    storage: &mut Storage<ID, ELEMENT>,
    id: ID,
    name: &str,
) -> Result<()> {
    if let Some(name) = Name::new(name) {
        if storage
            .get_all()
            .iter()
            .filter(|r| r.id().ne(&id))
            .any(|r| r.name().eq(&name))
        {
            bail!("Name '{}' already exists!", name)
        }

        storage
            .get_mut(id)
            .map(|r| r.set_name(name))
            .context("Id doesn't exist")
    } else {
        bail!("Name is empty!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::world::mountain::{Mountain, MountainId};
    use crate::utils::storage::Id;

    const VALID_NAME: &'static str = "Test";

    #[test]
    fn update_empty_name() {
        test_invalid_name("");
    }

    #[test]
    fn update_name_contains_only_whitespaces() {
        test_invalid_name("  ");
    }

    #[test]
    fn update_name_of_non_existing_mountain() {
        let mut storage: Storage<MountainId, Mountain> = Storage::default();

        assert!(update_name(&mut storage, MountainId::new(0), VALID_NAME).is_err());
    }

    #[test]
    fn update_valid_name() {
        test_update_name(VALID_NAME, VALID_NAME);
    }

    #[test]
    fn update_trimmed_name() {
        test_update_name(" Name ", "Name");
    }

    #[test]
    fn update_duplicate_name() {
        let mut storage: Storage<MountainId, Mountain> = Storage::default();
        let id0 = storage.create(Mountain::new);
        let id1 = storage.create(Mountain::new);

        assert!(update_name(&mut storage, id0, VALID_NAME).is_ok());
        assert!(update_name(&mut storage, id1, VALID_NAME).is_err());
    }

    fn test_invalid_name(name: &str) {
        let mut storage: Storage<MountainId, Mountain> = Storage::default();
        let id = storage.create(Mountain::new);

        assert!(update_name(&mut storage, id, name).is_err());
    }

    fn test_update_name(input: &str, result: &str) {
        let mut storage: Storage<MountainId, Mountain> = Storage::default();
        let id = storage.create(Mountain::new);

        assert!(update_name(&mut storage, id, input).is_ok());

        assert_eq!(
            result,
            &storage.get(id).map(|r| r.name()).unwrap().to_string()
        );
    }
}
