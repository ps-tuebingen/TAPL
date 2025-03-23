use super::Rule;
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Pest(Box<PestErr<Rule>>),
    MissingInput { expected: String },
    RemainingInput { rule: Rule },
    UnexpectedRule { found: Rule, expected: String },
    UnknownKw { kw: String },
    BadNumber { num: String },
}

impl Error {
    pub fn missing(exp: &str) -> Error {
        Error::MissingInput {
            expected: exp.to_owned(),
        }
    }

    pub fn remaining(r: Rule) -> Error {
        Error::RemainingInput { rule: r }
    }

    pub fn unexpected(r: Rule, expected: &str) -> Error {
        Error::UnexpectedRule {
            found: r,
            expected: expected.to_owned(),
        }
    }

    pub fn kw(kw: &str) -> Error {
        Error::UnknownKw { kw: kw.to_owned() }
    }

    pub fn num(num: &str) -> Error {
        Error::BadNumber {
            num: num.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Pest(err) => write!(f, "Error in Pest:\n{err}"),
            Error::MissingInput { expected } => write!(f, "Missing input {expected}"),
            Error::RemainingInput { rule } => write!(f, "Remaining input {rule:?}"),
            Error::UnexpectedRule { found, expected } => {
                write!(f, "Unexpected rule {found:?}, expected {expected}")
            }
            Error::UnknownKw { kw } => write!(f, "Unknown Keyword {kw}"),
            Error::BadNumber { num } => write!(f, "Canot parse number {num}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}
