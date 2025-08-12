use crate::{CouldNotCast, ElementNotFound};
use std::fmt;

#[derive(Debug)]
pub enum WebError {
    ElementNotFound(ElementNotFound),
    CouldNotCast(CouldNotCast),
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebError::ElementNotFound(enf) => enf.fmt(f),
            WebError::CouldNotCast(cnc) => cnc.fmt(f),
        }
    }
}

impl std::error::Error for WebError {}

impl From<ElementNotFound> for WebError {
    fn from(err: ElementNotFound) -> WebError {
        WebError::ElementNotFound(err)
    }
}
impl From<CouldNotCast> for WebError {
    fn from(err: CouldNotCast) -> WebError {
        WebError::CouldNotCast(err)
    }
}
