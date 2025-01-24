use crate::{
    syntax::{Label, Loc, Var},
    types::Type,
};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    UndefinedLabel(Label),
    UnassignedLocation(Loc),
    TypeMismatch(Type, Type),
    NoFunction(Type),
    NoList(Type),
    NoReference(Type),
    NoRecord(Type),
    EmptyCase,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(var) => write!(f, "Variable {var} appears free"),
            Error::UndefinedLabel(label) => write!(f, "Label {label} was never defined"),
            Error::UnassignedLocation(loc) => write!(f, "Location {loc} was never assigned"),
            Error::TypeMismatch(ty1, ty2) => write!(f, "Cannot unify {ty1} and {ty2}"),
            Error::NoList(ty) => write!(f, "{ty} should be a list type"),
            Error::NoReference(ty) => write!(f, "{ty} is not a reference"),
            Error::NoFunction(ty) => write!(f, "{ty} is not a function type"),
            Error::NoRecord(ty) => write!(f, "{ty} is not a record type"),
            Error::EmptyCase => write!(f, "Case cannot have zero patterns"),
        }
    }
}

impl std::error::Error for Error {}
