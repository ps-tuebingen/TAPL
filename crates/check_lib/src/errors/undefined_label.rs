use std::fmt;
use syntax::Label;

#[derive(Debug)]
pub struct UndefinedLabel {
    label: Label,
}

impl UndefinedLabel {
    pub fn new(lb: &str) -> UndefinedLabel {
        UndefinedLabel {
            label: lb.to_owned(),
        }
    }
}

impl fmt::Display for UndefinedLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined Label {}", self.label)
    }
}

impl std::error::Error for UndefinedLabel {}
