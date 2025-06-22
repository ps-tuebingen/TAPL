use check::errors::CheckError;
use common::errors::{NotImplemented, ValueMismatch};
use parse::{
    errors::{MissingInput, ParserError, RemainingInput, UnexpectedRule},
    Rule,
};
use pest::error::Error as PestErr;
use std::fmt;
use syntax::untyped::Untyped;

#[derive(Debug)]
pub enum Error {
    NotImplemented(NotImplemented),
    ValueMismatch(ValueMismatch),
    Parse(ParserError),
    Check(CheckError<Untyped>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotImplemented(ni) => ni.fmt(f),
            Error::ValueMismatch(vm) => vm.fmt(f),
            Error::Parse(p) => p.fmt(f),
            Error::Check(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<NotImplemented> for Error {
    fn from(err: NotImplemented) -> Error {
        Error::NotImplemented(err)
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

impl From<CheckError<Untyped>> for Error {
    fn from(ur: CheckError<Untyped>) -> Error {
        Error::Check(ur.into())
    }
}
