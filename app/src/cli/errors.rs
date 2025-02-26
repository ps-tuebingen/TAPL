use std::{fmt, io::Error as IOError};
use untyped_arithmetic::parse::errors::Error as ArithErr;

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    UntypedArith(ArithErr),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO error: {err}"),
            Error::UntypedArith(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IO(err)
    }
}

impl From<ArithErr> for Error {
    fn from(err: ArithErr) -> Error {
        Error::UntypedArith(err)
    }
}
