use crate::{
    DefinitionNotFound, IndexOutOfBounds, KindMismatch, NoTyping, TypeMismatch, UndefinedLabel,
    UndefinedMain,
};
use std::fmt;

#[derive(Debug)]
pub enum InferenceError {
    TypeMismatch(TypeMismatch),
    UndefinedLabel(UndefinedLabel),
    IndexOutOfBounds(IndexOutOfBounds),
    DefinitionNotFound(DefinitionNotFound),
    KindMismatch(KindMismatch),
    UndefinedMain(UndefinedMain),
    NoTyping(NoTyping),
}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TypeMismatch(tm) => tm.fmt(f),
            Self::UndefinedLabel(ul) => ul.fmt(f),
            Self::IndexOutOfBounds(ioob) => ioob.fmt(f),
            Self::DefinitionNotFound(dnf) => dnf.fmt(f),
            Self::KindMismatch(km) => km.fmt(f),
            Self::UndefinedMain(um) => um.fmt(f),
            Self::NoTyping(nt) => nt.fmt(f),
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

impl From<DefinitionNotFound> for InferenceError {
    fn from(dnf: DefinitionNotFound) -> Self {
        Self::DefinitionNotFound(dnf)
    }
}

impl From<KindMismatch> for InferenceError {
    fn from(km: KindMismatch) -> Self {
        Self::KindMismatch(km)
    }
}

impl From<UndefinedMain> for InferenceError {
    fn from(um: UndefinedMain) -> Self {
        Self::UndefinedMain(um)
    }
}

impl From<NoTyping> for InferenceError {
    fn from(nt: NoTyping) -> Self {
        Self::NoTyping(nt)
    }
}
