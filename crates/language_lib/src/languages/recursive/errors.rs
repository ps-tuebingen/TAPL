use check::errors::EmptyCase;
use common::errors::{
    FreeTypeVariable, FreeVariable, KindMismatch, NotImplemented, TypeMismatch, UndefinedLabel,
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
    TypeMismatch(TypeMismatch),
    KindMismatch(KindMismatch),
    UndefinedLabel(UndefinedLabel),
    EmptyCase(EmptyCase),
    NotImplemented(NotImplemented),
    ValueMismatch(ValueMismatch),
    FreeVariable(FreeVariable),
    Pest(Box<PestErr<Rule>>),
    Parse(ParserError),
    UnknownKeyword(UnknownKeyword),
    FreeTypeVariable(FreeTypeVariable),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TypeMismatch(tm) => tm.fmt(f),
            Error::KindMismatch(km) => km.fmt(f),
            Error::UndefinedLabel(ul) => ul.fmt(f),
            Error::EmptyCase(ec) => ec.fmt(f),
            Error::NotImplemented(ni) => ni.fmt(f),
            Error::ValueMismatch(vm) => vm.fmt(f),
            Error::FreeVariable(fv) => fv.fmt(f),
            Error::Pest(err) => err.fmt(f),
            Error::UnknownKeyword(uk) => uk.fmt(f),
            Error::FreeTypeVariable(fv) => fv.fmt(f),
            Error::Parse(p) => p.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

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

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}

impl From<ParserError> for Error {
    fn from(err: ParserError) -> Error {
        Error::Parse(err)
    }
}

impl From<UnknownKeyword> for Error {
    fn from(err: UnknownKeyword) -> Error {
        Error::UnknownKeyword(err)
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
