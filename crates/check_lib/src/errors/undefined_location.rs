use super::EnvError;
use std::fmt;
use syntax::Location;

#[derive(Debug)]
pub struct UndefinedLocation {
    loc: Location,
}

impl UndefinedLocation {
    pub fn new(loc: Location) -> UndefinedLocation {
        UndefinedLocation { loc }
    }
}

impl fmt::Display for UndefinedLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined Location {}", self.loc)
    }
}

impl From<UndefinedLocation> for EnvError {
    fn from(ul: UndefinedLocation) -> EnvError {
        EnvError::UndefinedLocation(ul)
    }
}

impl std::error::Error for UndefinedLocation {}
