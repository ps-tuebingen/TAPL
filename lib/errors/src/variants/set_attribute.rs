use std::fmt;

#[derive(Debug)]
pub struct SetAttribute {
    elem_id: String,
    attribute: String,
    value: String,
}

impl SetAttribute {
    pub fn new(id: &str, attr: &str, val: &str) -> SetAttribute {
        SetAttribute {
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
