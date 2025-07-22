use crate::{
    EmptyCase, FreeTypeVariable, FreeVariable, IndexOutOfBounds, KindMismatch, NameMismatch,
    NotASubtype, TypeMismatch, UndefinedLabel, UndefinedLocation,
};
use std::fmt;

#[derive(Debug)]
pub enum CheckError {
    EmptyCase(EmptyCase),
    NotASubtype(NotASubtype),
    KindMismatch(KindMismatch),
    TypeMismatch(TypeMismatch),
    UndefinedLocation(UndefinedLocation),
    IndexOutOfBounds(IndexOutOfBounds),
    FreeTypeVariable(FreeTypeVariable),
    UndefinedLabel(UndefinedLabel),
    NameMismatch(NameMismatch),
    FreeVariable(FreeVariable),
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CheckError::EmptyCase(ec) => ec.fmt(f),
            CheckError::NotASubtype(ns) => ns.fmt(f),
            CheckError::KindMismatch(km) => km.fmt(f),
            CheckError::TypeMismatch(tm) => tm.fmt(f),
            CheckError::UndefinedLocation(ul) => ul.fmt(f),
            CheckError::IndexOutOfBounds(io) => io.fmt(f),
            CheckError::FreeTypeVariable(fv) => fv.fmt(f),
            CheckError::UndefinedLabel(ul) => ul.fmt(f),
            CheckError::NameMismatch(nm) => nm.fmt(f),
            CheckError::FreeVariable(fv) => fv.fmt(f),
        }
    }
}

impl std::error::Error for CheckError {}

impl From<EmptyCase> for CheckError {
    fn from(err: EmptyCase) -> CheckError {
        CheckError::EmptyCase(err)
    }
}

impl From<NotASubtype> for CheckError {
    fn from(err: NotASubtype) -> CheckError {
        CheckError::NotASubtype(err)
    }
}

impl From<KindMismatch> for CheckError {
    fn from(err: KindMismatch) -> CheckError {
        CheckError::KindMismatch(err)
    }
}

impl From<TypeMismatch> for CheckError {
    fn from(err: TypeMismatch) -> CheckError {
        CheckError::TypeMismatch(err)
    }
}

impl From<UndefinedLocation> for CheckError {
    fn from(err: UndefinedLocation) -> CheckError {
        CheckError::UndefinedLocation(err)
    }
}

impl From<IndexOutOfBounds> for CheckError {
    fn from(err: IndexOutOfBounds) -> CheckError {
        CheckError::IndexOutOfBounds(err)
    }
}

impl From<FreeTypeVariable> for CheckError {
    fn from(err: FreeTypeVariable) -> CheckError {
        CheckError::FreeTypeVariable(err)
    }
}

impl From<UndefinedLabel> for CheckError {
    fn from(err: UndefinedLabel) -> CheckError {
        CheckError::UndefinedLabel(err)
    }
}

impl From<NameMismatch> for CheckError {
    fn from(err: NameMismatch) -> CheckError {
        CheckError::NameMismatch(err)
    }
}

impl From<FreeVariable> for CheckError {
    fn from(err: FreeVariable) -> CheckError {
        CheckError::FreeVariable(err)
    }
}
