use super::lexer::Token;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParenMismatch,
    UnexpectedEOI,
    UnexpectedToken(Token),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParenMismatch => f.write_str("Mismatched praentheses"),
            Error::UnexpectedEOI => f.write_str("Unexpected end of input."),
            Error::UnexpectedToken(tok) => write!(f, "Unexpected token {tok}."),
        }
    }
}

impl std::error::Error for Error {}
