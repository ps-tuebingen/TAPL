use std::fmt;

pub mod missing_input;
pub mod remaining_input;
pub mod unexpected_rule;
pub mod unknown_keyword;

pub use missing_input::MissingInput;
pub use remaining_input::RemainingInput;
pub use unexpected_rule::UnexpectedRule;
pub use unknown_keyword::UnknownKeyword;

#[derive(Debug)]
pub enum ParserError {
    MissingInput(MissingInput),
    RemainingInput(RemainingInput),
    UnexpectedRule(UnexpectedRule),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::MissingInput(mi) => mi.fmt(f),
            ParserError::RemainingInput(ri) => ri.fmt(f),
            ParserError::UnexpectedRule(ur) => ur.fmt(f),
        }
    }
}

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
