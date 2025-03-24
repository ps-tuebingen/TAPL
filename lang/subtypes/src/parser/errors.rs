use super::Rule;
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Pest(Box<PestErr<Rule>>),
    RemainingInput(Rule),
    MissingInput(String),
    UnexpectedRule { found: Rule, expected: String },
    UnknownKw(String),
    NotANumber(String),
    MissingCtor(String),
}

impl Error {
    pub fn unexpected(r: Rule, exp: &str) -> Error {
        Error::UnexpectedRule {
            found: r,
            expected: exp.to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Pest(err) => write!(f, "Error in Pest:{err}"),
            Error::RemainingInput(r) => write!(f, "Remaining Input {r:?}"),
            Error::MissingInput(missing) => write!(f, "Missing Input {missing}"),
            Error::UnexpectedRule { found, expected } => {
                write!(f, "Unexpected rule {found:?}, expected {expected}")
            }
            Error::UnknownKw(kw) => write!(f, "Unknown keyword {kw}"),
            Error::NotANumber(num) => write!(f, "Not a Number: {num}"),
            Error::MissingCtor(ctor) => write!(f, "Missing pattern for {ctor} in case"),
        }
    }
}

impl std::error::Error for Error {}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}
