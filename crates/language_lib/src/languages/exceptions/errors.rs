use common::errors::{FreeVariable, KindMismatch, NotImplemented, TypeMismatch, ValueMismatch};
use parse::{
    errors::{MissingInput, ParserError, RemainingInput, UnexpectedRule, UnknownKeyword},
    Rule,
};
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotImplemented(NotImplemented),
    KindMismatch(KindMismatch),
    TypeMismatch(TypeMismatch),
    FreeVariable(FreeVariable),
    ValueMismatch(ValueMismatch),
    Parse(ParserError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotImplemented(fv) => fv.fmt(f),
            Error::KindMismatch(km) => km.fmt(f),
            Error::TypeMismatch(tm) => tm.fmt(f),
            Error::FreeVariable(fv) => fv.fmt(f),
            Error::ValueMismatch(vm) => vm.fmt(f),
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
