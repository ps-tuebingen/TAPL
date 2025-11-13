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
            Self::Pest(err) => write!(f, "Error in Pest:\n{err}"),
            Self::MissingInput(mi) => mi.fmt(f),
            Self::RemainingInput(ri) => ri.fmt(f),
            Self::UnexpectedRule(ur) => ur.fmt(f),
            Self::UnknownKeyword(uk) => uk.fmt(f),
            Self::DuplicateDefinition(dd) => dd.fmt(f),
            Self::UndefinedMain(um) => um.fmt(f),
        }
    }
}

impl std::error::Error for ParserError {}

impl From<MissingInput> for ParserError {
    fn from(mi: MissingInput) -> Self {
        Self::MissingInput(mi)
    }
}

impl From<RemainingInput> for ParserError {
    fn from(ri: RemainingInput) -> Self {
        Self::RemainingInput(ri)
    }
}

impl From<UnexpectedRule> for ParserError {
    fn from(ur: UnexpectedRule) -> Self {
        Self::UnexpectedRule(ur)
    }
}

impl From<UnknownKeyword> for ParserError {
    fn from(ur: UnknownKeyword) -> Self {
        Self::UnknownKeyword(ur)
    }
}

impl<T> From<pest::error::Error<T>> for ParserError
where
    T: fmt::Debug,
{
    fn from(err: pest::error::Error<T>) -> Self {
        Self::Pest(format!("{err:?}"))
    }
}

impl From<DuplicateDefinition> for ParserError {
    fn from(err: DuplicateDefinition) -> Self {
        Self::DuplicateDefinition(err)
    }
}

impl From<UndefinedMain> for ParserError {
    fn from(err: UndefinedMain) -> Self {
        Self::UndefinedMain(err)
    }
}
