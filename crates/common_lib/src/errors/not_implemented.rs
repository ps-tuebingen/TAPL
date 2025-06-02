use std::fmt;

#[derive(Debug)]
pub struct NotImplemented;

impl fmt::Display for NotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not Implemented")
    }
}

impl std::error::Error for NotImplemented {}
