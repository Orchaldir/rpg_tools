use crate::model::character::Character;
use crate::model::name::Name;

pub fn get_str(name: Option<&Name>) -> &str {
    name.map(|n| n.str()).unwrap_or("")
}

pub fn get_first_name(character: &Character) -> &str {
    character.name.first().str()
}

pub fn get_middle_name(character: &Character) -> &str {
    get_str(character.name.middle())
}

pub fn get_last_name(character: &Character) -> &str {
    get_str(character.name.last().name())
}
