use crate::{FreeVariable, IndexOutOfBounds, UndefinedLabel, UndefinedLocation, ValueMismatch};
use std::fmt;

#[derive(Debug)]
pub enum EvalError {
    ValueMismatch(ValueMismatch),
    UndefinedLocation(UndefinedLocation),
    IndexOutOfBounds(IndexOutOfBounds),
    UndefinedLabel(UndefinedLabel),
    FreeVariable(FreeVariable),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ValueMismatch(vm) => vm.fmt(f),
            Self::UndefinedLocation(ul) => ul.fmt(f),
            Self::IndexOutOfBounds(err) => err.fmt(f),
            Self::UndefinedLabel(err) => err.fmt(f),
            Self::FreeVariable(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for EvalError {}

impl From<ValueMismatch> for EvalError {
    fn from(err: ValueMismatch) -> Self {
        Self::ValueMismatch(err)
    }
}

impl From<UndefinedLocation> for EvalError {
    fn from(err: UndefinedLocation) -> Self {
        Self::UndefinedLocation(err)
    }
}

impl From<IndexOutOfBounds> for EvalError {
    fn from(err: IndexOutOfBounds) -> Self {
        Self::IndexOutOfBounds(err)
    }
}

impl From<UndefinedLabel> for EvalError {
    fn from(err: UndefinedLabel) -> Self {
        Self::UndefinedLabel(err)
    }
}

impl From<FreeVariable> for EvalError {
    fn from(err: FreeVariable) -> Self {
        Self::FreeVariable(err)
    }
}
