use super::Rule;
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Pest(PestErr<Rule>),
    EmptyInput,
    RemainingInput(Rule),
    MissingInput(String),
    UnexpectedRule { found: Rule, expected: Rule },
    BadType(String),
    BadTerm(String),
    BadRule(Rule),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Pest(err) => write!(f, "Error in Pest: {err}"),
            Error::EmptyInput => f.write_str("No rule matched input"),
            Error::RemainingInput(rule) => write!(f, "Remaining input {rule:?} after parsing"),
            Error::MissingInput(msg) => write!(f, "Missing input: {msg}"),
            Error::UnexpectedRule { found, expected } => {
                write!(f, "Unexpected rule: {found:?}, expected {expected:?}")
            }
            Error::BadType(ty) => write!(f, "{ty} is not a type"),
            Error::BadTerm(t) => write!(f, "{t} is not a term"),
            Error::BadRule(r) => write!(f, "Cannot parse rule {r:?}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(err)
    }
}
