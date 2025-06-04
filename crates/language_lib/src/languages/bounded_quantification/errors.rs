use super::{parser::Rule, types::Type};
use check::errors::NotASubtype;
use common::{
    errors::{
        FreeTypeVariable, FreeVariable, KindMismatch, NameMismatch, NotImplemented, TypeMismatch,
        UndefinedLabel,
    },
    parse::{MissingInput, RemainingInput, UnexpectedRule, UnknownKeyword},
};
use eval::errors::ValueMismatch;
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeTypeVariable(FreeTypeVariable),
    NotImplemented(NotImplemented),
    TypeMismatch(TypeMismatch),
    KindMismatch(KindMismatch),
    NameMismatch(NameMismatch),
    UndefinedLabel(UndefinedLabel),
    NotASubtype(NotASubtype<Type, Type>),
    FreeVariable(FreeVariable),
    ValueMismatch(ValueMismatch),
    Pest(PestErr<Rule>),
    MissingInput(MissingInput),
    RemainingInput(RemainingInput),
    UnexpectedRule(UnexpectedRule<Rule>),
    UnknownKeyword(UnknownKeyword),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeTypeVariable(fv) => fv.fmt(f),
            Error::NotImplemented(ni) => ni.fmt(f),
            Error::TypeMismatch(tm) => tm.fmt(f),
            Error::KindMismatch(km) => km.fmt(f),
            Error::NameMismatch(nm) => nm.fmt(f),
            Error::UndefinedLabel(ul) => ul.fmt(f),
            Error::NotASubtype(ns) => ns.fmt(f),
            Error::FreeVariable(fv) => fv.fmt(f),
            Error::ValueMismatch(vm) => vm.fmt(f),
            Error::Pest(err) => err.fmt(f),
            Error::MissingInput(mi) => mi.fmt(f),
            Error::RemainingInput(ri) => ri.fmt(f),
            Error::UnexpectedRule(ur) => ur.fmt(f),
            Error::UnknownKeyword(uk) => uk.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<FreeTypeVariable> for Error {
    fn from(err: FreeTypeVariable) -> Error {
        Error::FreeTypeVariable(err)
    }
}

impl From<NotImplemented> for Error {
    fn from(err: NotImplemented) -> Error {
        Error::NotImplemented(err)
    }
}

impl From<TypeMismatch> for Error {
    fn from(err: TypeMismatch) -> Error {
        Error::TypeMismatch(err)
    }
}

impl From<KindMismatch> for Error {
    fn from(err: KindMismatch) -> Error {
        Error::KindMismatch(err)
    }
}

impl From<NameMismatch> for Error {
    fn from(err: NameMismatch) -> Error {
        Error::NameMismatch(err)
    }
}

impl From<UndefinedLabel> for Error {
    fn from(err: UndefinedLabel) -> Error {
        Error::UndefinedLabel(err)
    }
}

impl From<NotASubtype<Type, Type>> for Error {
    fn from(err: NotASubtype<Type, Type>) -> Error {
        Error::NotASubtype(err)
    }
}

impl From<FreeVariable> for Error {
    fn from(err: FreeVariable) -> Error {
        Error::FreeVariable(err)
    }
}

impl From<ValueMismatch> for Error {
    fn from(err: ValueMismatch) -> Error {
        Error::ValueMismatch(err)
    }
}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(err)
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

impl From<UnknownKeyword> for Error {
    fn from(err: UnknownKeyword) -> Error {
        Error::UnknownKeyword(err)
    }
}
