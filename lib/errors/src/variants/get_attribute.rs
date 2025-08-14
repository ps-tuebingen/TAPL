use std::fmt;

#[derive(Debug)]
pub struct GetAttribute {
    element_id: String,
    attribute: String,
}

impl GetAttribute {
    pub fn new(id: &str, attr: &str) -> GetAttribute {
        GetAttribute {
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
