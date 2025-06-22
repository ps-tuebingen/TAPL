use common::errors::{
    FreeTypeVariable, FreeVariable, KindMismatch, NotImplemented, TypeMismatch, ValueMismatch,
};
use parse::{
    Rule,
    errors::{MissingInput, ParserError, RemainingInput, UnexpectedRule},
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
    Parse(ParserError),
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
            Error::Parse(p) => p.fmt(f),
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
        Error::Parse(err.into())
    }
}

impl From<ParserError> for Error {
    fn from(err: ParserError) -> Error {
        Error::Parse(err)
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
