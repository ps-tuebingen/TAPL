use super::Value;
use crate::{Label, Var};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVariable { var: Var },
    BadValue { val: Value },
    UndefinedLabel { label: Label },
    MissingPattern { label: Label },
    ProjectionOutOfBounds { found: usize, expected: usize },
    HeadOfEmptyList,
    TailOfEmptyList,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVariable { var } => write!(f, "Variable {var} should not appear free."),
            Error::BadValue { val } => write!(f, "Value {val} cannot appear in this position."),
            Error::UndefinedLabel { label } => write!(f, "Label {label} was defined but not used."),
            Error::MissingPattern { label } => write!(f, "No pattern for label {label}"),
            Error::ProjectionOutOfBounds { found, expected } => write!(
                f,
                "Cannot perform projection {expected} with {found} number of terms."
            ),
            Error::HeadOfEmptyList => f.write_str("Cannot take head of empty list."),
            Error::TailOfEmptyList => f.write_str("Cannot take tail of empty list."),
        }
    }
}

impl std::error::Error for Error {}
