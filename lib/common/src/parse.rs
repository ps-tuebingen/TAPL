use crate::errors::{Error, ErrorKind, ErrorLocation};

pub trait Parse: Sized {
    fn parse(sourcte: String) -> Result<Self, Error>;
}

pub fn to_parse_err<T>(knd: T) -> Error
where
    T: Into<ErrorKind>,
{
    Error {
        loc: ErrorLocation::Parse,
        kind: knd.into(),
    }
}
