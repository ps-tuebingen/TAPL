use std::fmt;

pub mod kind;
pub mod loc;
pub use kind::ErrorKind;
pub use loc::ErrorLocation;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub loc: ErrorLocation,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error during {} :\n{}", self.loc, self.kind)
    }
}

impl std::error::Error for Error {}
