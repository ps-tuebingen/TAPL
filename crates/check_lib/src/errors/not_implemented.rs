use super::EnvError;
use std::fmt;

#[derive(Debug)]
pub struct NotImplemented;

impl fmt::Display for NotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not Implemented")
    }
}

impl From<NotImplemented> for EnvError {
    fn from(ni: NotImplemented) -> EnvError {
        EnvError::NotImplemented(ni)
    }
}

impl std::error::Error for NotImplemented {}
