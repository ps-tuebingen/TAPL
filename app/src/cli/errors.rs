use std::{fmt, io::Error as IOError};
use untyped_arithmetic::parse::errors::Error as ArithErr;
use untyped_lambda::parse::errors::Error as LamErr;

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    UntypedArith(ArithErr),
    UntypedLam(LamErr),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO error: {err}"),
            Error::UntypedArith(err) => err.fmt(f),
            Error::UntypedLam(err) => err.fmt(f),
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

impl From<LamErr> for Error {
    fn from(err: LamErr) -> Error {
        Error::UntypedLam(err)
    }
}
