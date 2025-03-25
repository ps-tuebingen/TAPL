use super::Rule;
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Pest(Box<PestErr<Rule>>),
    MissingInput(String),
    RemainingInput(Rule),
    UnexpectedRule { found: Rule, expected: String },
    MultipleConstructors { class_name: String },
    UnknownVariable(String),
    NotANumber(String),
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
            Error::MissingInput(missing) => write!(f, "Missing Input {missing}"),
            Error::RemainingInput(r) => write!(f, "Remaining Input {r:?}"),
            Error::UnexpectedRule { found, expected } => {
                write!(f, "Unexpected rule {found:?}, expected {expected}")
            }
            Error::MultipleConstructors { class_name } => write!(
                f,
                "Multiple Constructor Declarations for class {class_name}"
            ),
            Error::UnknownVariable(var) => write!(f, "Unkown Variable {var}"),
            Error::NotANumber(s) => write!(f, "{s} is not a number"),
        }
    }
}

impl std::error::Error for Error {}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}
