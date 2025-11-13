use std::fmt;

#[derive(Debug)]
pub struct UndefinedLabel {
    label: String,
}

impl UndefinedLabel {
    #[must_use] pub fn new(lb: &str) -> Self {
        Self {
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
