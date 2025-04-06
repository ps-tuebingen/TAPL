use super::Rule;
use pest::{error::Error as PestErr, iterators::Pair};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Pest(Box<PestErr<Rule>>),
    MissingInput(String),
    RemainingInput(Rule),
    UnexpectedRule { found: Rule, expected: String },
    UnknownKeyword(String),
}

impl Error {
    pub fn missing(miss: &str) -> Error {
        Error::MissingInput(miss.to_owned())
    }

    pub fn remaining(p: &Pair<'_, Rule>) -> Error {
        Error::RemainingInput(p.as_rule())
    }

    pub fn unexpected(found: &Pair<'_, Rule>, exp: &str) -> Error {
        Error::UnexpectedRule {
            found: found.as_rule(),
            expected: exp.to_owned(),
        }
    }

    pub fn unknown(found: &str) -> Error {
        Error::UnknownKeyword(found.to_owned())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Pest(err) => write!(f, "Error in Pest: {err}"),
            Error::MissingInput(missing) => write!(f, "Missing Input {missing}"),
            Error::RemainingInput(remaining) => write!(f, "Remaining Input {remaining:?}"),
            Error::UnexpectedRule { found, expected } => {
                write!(f, "Unexpected Rule {found:?}, expected {expected}")
            }
            Error::UnknownKeyword(found) => write!(f, "Unknown Keyword {found}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}
