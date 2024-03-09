use crate::model::name::Name;

pub mod town;
pub mod towns;

pub fn get_str(name: Option<&Name>) -> &str {
    name.map(|n| n.str()).unwrap_or("")
}
