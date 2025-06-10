use super::parser::Rule;
use common::{
    errors::{
        FreeTypeVariable, FreeVariable, KindMismatch, NotImplemented, TypeMismatch, ValueMismatch,
    },
    parse::{MissingInput, RemainingInput, UnexpectedRule},
};
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotImplemented(NotImplemented),
    FreeTypeVariable(FreeTypeVariable),
    KindMismatch(KindMismatch),
    TypeMismatch(TypeMismatch),
    ValueMismatch(ValueMismatch),
    FreeVariable(FreeVariable),
    Pest(Box<PestErr<Rule>>),
    MissingInput(MissingInput),
    RemainingInput(RemainingInput),
    UnexpectedRule(UnexpectedRule<Rule>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotImplemented(ni) => ni.fmt(f),
            Error::FreeTypeVariable(fv) => fv.fmt(f),
            Error::KindMismatch(km) => km.fmt(f),
            Error::TypeMismatch(tm) => tm.fmt(f),
            Error::ValueMismatch(vm) => vm.fmt(f),
            Error::FreeVariable(fv) => fv.fmt(f),
            Error::Pest(err) => err.fmt(f),
            Error::MissingInput(mi) => mi.fmt(f),
            Error::RemainingInput(ri) => ri.fmt(f),
            Error::UnexpectedRule(ur) => ur.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<NotImplemented> for Error {
    fn from(err: NotImplemented) -> Error {
        Error::NotImplemented(err)
    }
}

impl From<FreeTypeVariable> for Error {
    fn from(err: FreeTypeVariable) -> Error {
        Error::FreeTypeVariable(err)
    }
}

impl From<KindMismatch> for Error {
    fn from(err: KindMismatch) -> Error {
        Error::KindMismatch(err)
    }
}

impl From<TypeMismatch> for Error {
    fn from(err: TypeMismatch) -> Error {
        Error::TypeMismatch(err)
    }
}

impl From<ValueMismatch> for Error {
    fn from(err: ValueMismatch) -> Error {
        Error::ValueMismatch(err)
    }
}

impl From<FreeVariable> for Error {
    fn from(err: FreeVariable) -> Error {
        Error::FreeVariable(err)
    }
}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}

impl From<MissingInput> for Error {
    fn from(err: MissingInput) -> Error {
        Error::MissingInput(err)
    }
}

impl From<RemainingInput> for Error {
    fn from(err: RemainingInput) -> Error {
        Error::RemainingInput(err)
    }
}

impl From<UnexpectedRule<Rule>> for Error {
    fn from(err: UnexpectedRule<Rule>) -> Error {
        Error::UnexpectedRule(err)
    }
}
