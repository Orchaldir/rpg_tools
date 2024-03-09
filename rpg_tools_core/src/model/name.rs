use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// A valid name.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Name {
    text: String,
}

impl Name {
    /// Returns a new name, if valid.
    ///
    /// ```
    ///# use rpg_tools_core::model::name::Name;
    ///
    /// assert_eq!(Name::new(""), None);
    /// assert_eq!(Name::new("  "), None);
    /// assert_eq!(Name::new("Test0").unwrap().to_string(), "Test0");
    /// assert_eq!(Name::new("  Test1  ").unwrap().to_string(), "Test1");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Option<Name> {
        let trimmed = name.into().trim().to_string();

        if trimmed.is_empty() {
            None
        } else {
            Some(Self { text: trimmed })
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
