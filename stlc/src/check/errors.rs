use crate::{syntax::Term, types::Type, Label, Var};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    WrongAscription { found: Type, expected: Type },
    UnboundVariable { var: Var },
    UndefinedLabel { label: Label },
    UnexpectedType { ty: Type, term: Term },
    WrongNumberOfCases { found: usize, expected: usize },
    TypeMismatch { types: Vec<Type> },
    ProjectionOutOfBounds { proj_ty: Type, ind: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::WrongAscription { found, expected } => write!(
                f,
                "Bad Ascription for term: expected {expected}, found {found}."
            ),
            Error::UnboundVariable { var } => write!(f, "Variable {var} used but never bound."),
            Error::UndefinedLabel { label } => {
                write!(f, "Label {label} was used but never defined.")
            }
            Error::UnexpectedType { ty, term } => write!(f, "Term {term} cannot have type {ty}."),
            Error::WrongNumberOfCases { found, expected } => write!(
                f,
                "Wrong number of cases, found {found}, expected {expected}."
            ),
            Error::TypeMismatch { types } => {
                write!(
                    f,
                    "types {} should all be equal, but are not.",
                    types
                        .iter()
                        .map(|ty| format!("{ty}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Error::ProjectionOutOfBounds { proj_ty, ind } => write!(
                f,
                "Index {ind} is out of bounds for projection of type {proj_ty}."
            ),
        }
    }
}
