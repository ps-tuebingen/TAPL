use super::Rule;
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Pest(Box<PestErr<Rule>>),
    MissingInput(String),
    RemainingInput(Rule),
    UnexpectedRule { found: Rule, expected: String },
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
            Error::Pest(err) => write!(f, "Error in Pest: {err}"),
            Error::MissingInput(missing) => write!(f, "Missing Input {missing}"),
            Error::RemainingInput(rule) => write!(f, "Remaining Input {rule:?}"),
            Error::UnexpectedRule { found, expected } => {
                write!(f, "Unexpected Rule {found:?}, expected: {expected}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}
