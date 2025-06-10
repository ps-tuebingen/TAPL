use super::parse::lexer::Token;
use common::{
    errors::{FreeVariable, NotImplemented, ValueMismatch},
    parse::{MissingInput, UnexpectedRule},
};
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotImplemented(NotImplemented),
    ValueMismatch(ValueMismatch),
    FreeVariable(FreeVariable),
    Pest(Box<PestErr<NotImplemented>>),
    MissingInput(MissingInput),
    UnexpectedRule(UnexpectedRule<Token>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotImplemented(ni) => ni.fmt(f),
            Error::ValueMismatch(vm) => vm.fmt(f),
            Error::FreeVariable(fv) => fv.fmt(f),
            Error::Pest(_) => panic!("Impossible"),
            Error::MissingInput(mi) => mi.fmt(f),
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

impl From<PestErr<NotImplemented>> for Error {
    fn from(err: PestErr<NotImplemented>) -> Error {
        Error::Pest(Box::new(err))
    }
}

impl From<MissingInput> for Error {
    fn from(err: MissingInput) -> Error {
        Error::MissingInput(err)
    }
}

impl From<UnexpectedRule<Token>> for Error {
    fn from(err: UnexpectedRule<Token>) -> Error {
        Error::UnexpectedRule(err)
    }
}
