use super::Rule;
use errors::DuplicateDefinition;
use pest::error::Error as PestErr;
use std::fmt;

pub mod missing_input;
pub mod remaining_input;
pub mod undefined_main;
pub mod unexpected_rule;
pub mod unknown_keyword;

pub use missing_input::MissingInput;
pub use remaining_input::RemainingInput;
pub use undefined_main::UndefinedMain;
pub use unexpected_rule::UnexpectedRule;
pub use unknown_keyword::UnknownKeyword;

#[derive(Debug)]
pub enum ParserError {
    Pest(Box<PestErr<Rule>>),
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

impl From<PestErr<Rule>> for ParserError {
    fn from(err: PestErr<Rule>) -> ParserError {
        ParserError::Pest(Box::new(err))
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
