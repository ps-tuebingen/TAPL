use crate::{
    EmptyCase, FreeTypeVariable, FreeVariable, IndexOutOfBounds, KindMismatch, NameMismatch,
    NoKinding, NoSubtyping, NoTyping, NotASubtype, TypeMismatch, UndefinedLabel, UndefinedLocation,
    UnexpectedDerivation,
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
    UnexpectedDerivation(UnexpectedDerivation),
    NoSubtyping(NoSubtyping),
    NoTyping(NoTyping),
    NoKinding(NoKinding),
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyCase(ec) => ec.fmt(f),
            Self::NotASubtype(ns) => ns.fmt(f),
            Self::KindMismatch(km) => km.fmt(f),
            Self::TypeMismatch(tm) => tm.fmt(f),
            Self::UndefinedLocation(ul) => ul.fmt(f),
            Self::IndexOutOfBounds(io) => io.fmt(f),
            Self::FreeTypeVariable(fv) => fv.fmt(f),
            Self::UndefinedLabel(ul) => ul.fmt(f),
            Self::NameMismatch(nm) => nm.fmt(f),
            Self::FreeVariable(fv) => fv.fmt(f),
            Self::UnexpectedDerivation(ud) => ud.fmt(f),
            Self::NoSubtyping(ns) => ns.fmt(f),
            Self::NoTyping(nt) => nt.fmt(f),
            Self::NoKinding(nk) => nk.fmt(f),
        }
    }
}

impl std::error::Error for CheckError {}

impl From<EmptyCase> for CheckError {
    fn from(err: EmptyCase) -> Self {
        Self::EmptyCase(err)
    }
}

impl From<NotASubtype> for CheckError {
    fn from(err: NotASubtype) -> Self {
        Self::NotASubtype(err)
    }
}

impl From<KindMismatch> for CheckError {
    fn from(err: KindMismatch) -> Self {
        Self::KindMismatch(err)
    }
}

impl From<TypeMismatch> for CheckError {
    fn from(err: TypeMismatch) -> Self {
        Self::TypeMismatch(err)
    }
}

impl From<UndefinedLocation> for CheckError {
    fn from(err: UndefinedLocation) -> Self {
        Self::UndefinedLocation(err)
    }
}

impl From<IndexOutOfBounds> for CheckError {
    fn from(err: IndexOutOfBounds) -> Self {
        Self::IndexOutOfBounds(err)
    }
}

impl From<FreeTypeVariable> for CheckError {
    fn from(err: FreeTypeVariable) -> Self {
        Self::FreeTypeVariable(err)
    }
}

impl From<UndefinedLabel> for CheckError {
    fn from(err: UndefinedLabel) -> Self {
        Self::UndefinedLabel(err)
    }
}

impl From<NameMismatch> for CheckError {
    fn from(err: NameMismatch) -> Self {
        Self::NameMismatch(err)
    }
}

impl From<FreeVariable> for CheckError {
    fn from(err: FreeVariable) -> Self {
        Self::FreeVariable(err)
    }
}

impl From<UnexpectedDerivation> for CheckError {
    fn from(err: UnexpectedDerivation) -> Self {
        Self::UnexpectedDerivation(err)
    }
}

impl From<NoSubtyping> for CheckError {
    fn from(err: NoSubtyping) -> Self {
        Self::NoSubtyping(err)
    }
}

impl From<NoTyping> for CheckError {
    fn from(err: NoTyping) -> Self {
        Self::NoTyping(err)
    }
}

impl From<NoKinding> for CheckError {
    fn from(err: NoKinding) -> Self {
        Self::NoKinding(err)
    }
}
