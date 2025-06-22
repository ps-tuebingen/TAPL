use super::types::Type;
use check::errors::CheckError;
use check::errors::{EmptyCase, NotASubtype};
use common::errors::{
    FreeTypeVariable, FreeVariable, KindMismatch, TypeMismatch, UndefinedLabel, UndefinedLocation,
    ValueMismatch,
};
use parse::{
    errors::{MissingInput, ParserError, RemainingInput, UnexpectedRule, UnknownKeyword},
    Rule,
};
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UndefinedLocation(UndefinedLocation),
    TypeMismatch(TypeMismatch),
    KindMismatch(KindMismatch),
    UndefinedLabel(UndefinedLabel),
    EmptyCase(EmptyCase),
    NotASubtype(NotASubtype<Type>),
    FreeVariable(FreeVariable),
    ValueMismatch(ValueMismatch),
    Parse(ParserError),
    FreeTypeVariable(FreeTypeVariable),
    Check(CheckError<Type>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UndefinedLocation(loc) => loc.fmt(f),
            Error::TypeMismatch(tm) => tm.fmt(f),
            Error::KindMismatch(km) => km.fmt(f),
            Error::UndefinedLabel(ul) => ul.fmt(f),
            Error::EmptyCase(ec) => ec.fmt(f),
            Error::NotASubtype(ns) => ns.fmt(f),
            Error::FreeVariable(fv) => fv.fmt(f),
            Error::ValueMismatch(vm) => vm.fmt(f),
            Error::FreeTypeVariable(fv) => fv.fmt(f),
            Error::Parse(p) => p.fmt(f),
            Error::Check(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<UndefinedLocation> for Error {
    fn from(err: UndefinedLocation) -> Error {
        Error::UndefinedLocation(err)
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

impl From<UndefinedLabel> for Error {
    fn from(err: UndefinedLabel) -> Error {
        Error::UndefinedLabel(err)
    }
}

impl From<EmptyCase> for Error {
    fn from(err: EmptyCase) -> Error {
        Error::EmptyCase(err)
    }
}

impl From<NotASubtype<Type>> for Error {
    fn from(err: NotASubtype<Type>) -> Error {
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
        Error::Parse(err.into())
    }
}
impl From<ParserError> for Error {
    fn from(err: ParserError) -> Error {
        Error::Parse(err)
    }
}

impl From<UnknownKeyword> for Error {
    fn from(err: UnknownKeyword) -> Error {
        Error::Parse(err.into())
    }
}

impl From<FreeTypeVariable> for Error {
    fn from(err: FreeTypeVariable) -> Error {
        Error::FreeTypeVariable(err)
    }
}

impl From<MissingInput> for Error {
    fn from(mi: MissingInput) -> Error {
        Error::Parse(mi.into())
    }
}

impl From<RemainingInput> for Error {
    fn from(ri: RemainingInput) -> Error {
        Error::Parse(ri.into())
    }
}

impl From<UnexpectedRule> for Error {
    fn from(ur: UnexpectedRule) -> Error {
        Error::Parse(ur.into())
    }
}

impl From<CheckError<Type>> for Error {
    fn from(ur: CheckError<Type>) -> Error {
        Error::Check(ur.into())
    }
}
