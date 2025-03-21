use super::Rule;
use pest::error::Error as PestErr;
use std::fmt;
#[derive(Debug)]
pub enum Error {
    Pest(Box<PestErr<Rule>>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Pest(err) => write!(f, "Error in Pest:\n{err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}
