use crate::{
    DuplicateDefinition, MissingInput, RemainingInput, UndefinedMain, UnexpectedRule,
    UnknownKeyword,
};
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    Pest(String),
    MissingInput(MissingInput),
    RemainingInput(RemainingInput),
    UnexpectedRule(UnexpectedRule),
    UnknownKeyword(UnknownKeyword),
    DuplicateDefinition(DuplicateDefinition),
    UndefinedMain(UndefinedMain),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::Pest(err) => write!(f, "Error in Pest:\n{err}"),
            ParserError::MissingInput(mi) => mi.fmt(f),
            ParserError::RemainingInput(ri) => ri.fmt(f),
            ParserError::UnexpectedRule(ur) => ur.fmt(f),
            ParserError::UnknownKeyword(uk) => uk.fmt(f),
            ParserError::DuplicateDefinition(dd) => dd.fmt(f),
            ParserError::UndefinedMain(um) => um.fmt(f),
        }
    }
}

impl std::error::Error for ParserError {}

impl From<MissingInput> for ParserError {
    fn from(mi: MissingInput) -> ParserError {
        ParserError::MissingInput(mi)
    }
}

impl From<RemainingInput> for ParserError {
    fn from(ri: RemainingInput) -> ParserError {
        ParserError::RemainingInput(ri)
    }
}

impl From<UnexpectedRule> for ParserError {
    fn from(ur: UnexpectedRule) -> ParserError {
        ParserError::UnexpectedRule(ur)
    }
}

impl From<UnknownKeyword> for ParserError {
    fn from(ur: UnknownKeyword) -> ParserError {
        ParserError::UnknownKeyword(ur)
    }
}

impl<T> From<pest::error::Error<T>> for ParserError
where
    T: fmt::Debug,
{
    fn from(err: pest::error::Error<T>) -> ParserError {
        ParserError::Pest(format!("{err:?}"))
    }
}

impl From<DuplicateDefinition> for ParserError {
    fn from(err: DuplicateDefinition) -> ParserError {
        ParserError::DuplicateDefinition(err)
    }
}

impl From<UndefinedMain> for ParserError {
    fn from(err: UndefinedMain) -> ParserError {
        ParserError::UndefinedMain(err)
    }
}
