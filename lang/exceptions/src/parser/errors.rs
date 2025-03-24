use super::Rule;
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Pest(Box<PestErr<Rule>>),
    RemainingInput(Rule),
    MissingInput(String),
    UnexpectedRule { found: Rule, expected: String },
    NotANumber(String),
    UnknownKw(String),
}

impl Error {
    pub fn unexpected(found: Rule, expected: &str) -> Error {
        Error::UnexpectedRule {
            found,
            expected: expected.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Pest(err) => write!(f, "Error in Pest: {err}"),
            Error::RemainingInput(r) => write!(f, "Remaining Input {r:?} after parsing"),
            Error::MissingInput(miss) => write!(f, "Missing Input {miss}"),
            Error::UnexpectedRule { found, expected } => {
                write!(f, "Unexpected rule: {found:?}, expected: {expected}")
            }
            Error::NotANumber(s) => write!(f, "{s} is not a number"),
            Error::UnknownKw(kw) => write!(f, "{kw} is not a keyword"),
        }
    }
}

impl std::error::Error for Error {}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}
