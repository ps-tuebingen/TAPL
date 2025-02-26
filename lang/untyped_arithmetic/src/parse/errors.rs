use super::lexer::Token;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnexpectedEOI,
    UnexpectedString(String),
    UnexpectedChar(char),
    UnexpectedToken(Token),
    RemainingInput(Vec<Token>),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnexpectedEOI => f.write_str("Unexpected End of Input"),
            Error::UnexpectedString(s) => write!(f, "Unexpected input {s}"),
            Error::UnexpectedChar(c) => write!(f, "Unexpected character {c}"),
            Error::UnexpectedToken(tok) => write!(f, "Unexpected Token {tok}"),
            Error::RemainingInput(tokens) => write!(
                f,
                "Remaining tokens after parsing term: {}",
                tokens
                    .iter()
                    .map(|tok| format!("{}", tok))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}
impl std::error::Error for Error {}
