use errors::{FreeVariable, IndexOutOfBounds, UndefinedLabel, UndefinedLocation, ValueMismatch};
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
            EvalError::ValueMismatch(vm) => vm.fmt(f),
            EvalError::UndefinedLocation(ul) => ul.fmt(f),
            EvalError::IndexOutOfBounds(err) => err.fmt(f),
            EvalError::UndefinedLabel(err) => err.fmt(f),
            EvalError::FreeVariable(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for EvalError {}

impl From<ValueMismatch> for EvalError {
    fn from(err: ValueMismatch) -> EvalError {
        EvalError::ValueMismatch(err)
    }
}

impl From<UndefinedLocation> for EvalError {
    fn from(err: UndefinedLocation) -> EvalError {
        EvalError::UndefinedLocation(err)
    }
}

impl From<IndexOutOfBounds> for EvalError {
    fn from(err: IndexOutOfBounds) -> EvalError {
        EvalError::IndexOutOfBounds(err)
    }
}

impl From<UndefinedLabel> for EvalError {
    fn from(err: UndefinedLabel) -> EvalError {
        EvalError::UndefinedLabel(err)
    }
}

impl From<FreeVariable> for EvalError {
    fn from(err: FreeVariable) -> EvalError {
        EvalError::FreeVariable(err)
    }
}
