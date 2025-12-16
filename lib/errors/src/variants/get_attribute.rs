use std::fmt;

/// Error getting html attribute
#[derive(Debug)]
pub struct GetAttribute {
    /// id of the element
    element_id: String,
    /// attribute being read
    attribute: String,
}

impl GetAttribute {
    /// Create a new error from id and attribute
    #[must_use]
    pub fn new(id: &str, attr: &str) -> Self {
        Self {
            element_id: id.to_owned(),
            attribute: attr.to_owned(),
        }
    }
}

impl fmt::Display for GetAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not get attribute {} of {}",
            self.attribute, self.element_id
        )
    }
}

impl std::error::Error for GetAttribute {}
