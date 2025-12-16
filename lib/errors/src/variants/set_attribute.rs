use std::fmt;

/// Error setting html attribute
#[derive(Debug)]
pub struct SetAttribute {
    /// Id of the element
    elem_id: String,
    /// attribute being set
    attribute: String,
    /// new value to set
    value: String,
}

impl SetAttribute {
    /// Create a new error from id, attribute and value
    #[must_use]
    pub fn new(id: &str, attr: &str, val: &str) -> Self {
        Self {
            elem_id: id.to_owned(),
            attribute: attr.to_owned(),
            value: val.to_owned(),
        }
    }
}

impl fmt::Display for SetAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not set attribute {}={} for {}",
            self.attribute, self.value, self.elem_id
        )
    }
}

impl std::error::Error for SetAttribute {}
