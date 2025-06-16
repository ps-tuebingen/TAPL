use common::errors::{NotImplemented, ValueMismatch};
use parse::{
    errors::{MissingInput, RemainingInput, UnexpectedRule},
    Rule,
};
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotImplemented(NotImplemented),
    ValueMismatch(ValueMismatch),
    Pest(Box<PestErr<Rule>>),
    RemainingInput(RemainingInput),
    UnexpectedRule(UnexpectedRule),
    MissingInput(MissingInput),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotImplemented(ni) => ni.fmt(f),
            Error::ValueMismatch(vm) => vm.fmt(f),
            Error::Pest(_) => panic!("Impossible"),
            Error::RemainingInput(ri) => ri.fmt(f),
            Error::UnexpectedRule(ur) => ur.fmt(f),
            Error::MissingInput(mi) => mi.fmt(f),
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
        Error::Pest(Box::new(err))
    }
}

impl From<RemainingInput> for Error {
    fn from(err: RemainingInput) -> Error {
        Error::RemainingInput(err)
    }
}

impl From<UnexpectedRule> for Error {
    fn from(err: UnexpectedRule) -> Error {
        Error::UnexpectedRule(err)
    }
}

impl From<MissingInput> for Error {
    fn from(err: MissingInput) -> Error {
        Error::MissingInput(err)
    }
}
