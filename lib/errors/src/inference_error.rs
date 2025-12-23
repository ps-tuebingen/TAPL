use crate::{IndexOutOfBounds, TypeMismatch, UndefinedLabel};
use std::fmt;

#[derive(Debug)]
pub enum InferenceError {
    TypeMismatch(TypeMismatch),
    UndefinedLabel(UndefinedLabel),
    IndexOutOfBounds(IndexOutOfBounds),
}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TypeMismatch(tm) => tm.fmt(f),
            Self::UndefinedLabel(ul) => ul.fmt(f),
            Self::IndexOutOfBounds(ioob) => ioob.fmt(f),
        }
    }
}

impl std::error::Error for InferenceError {}

impl From<TypeMismatch> for InferenceError {
    fn from(tm: TypeMismatch) -> Self {
        Self::TypeMismatch(tm)
    }
}

impl From<UndefinedLabel> for InferenceError {
    fn from(ul: UndefinedLabel) -> Self {
        Self::UndefinedLabel(ul)
    }
}

impl From<IndexOutOfBounds> for InferenceError {
    fn from(ioob: IndexOutOfBounds) -> Self {
        Self::IndexOutOfBounds(ioob)
    }
}
