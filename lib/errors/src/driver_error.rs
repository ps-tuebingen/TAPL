use crate::{
    FileAccess, NoTyping, UndefinedLanguage, check_error::CheckError, eval_error::EvalError,
    parse_error::ParserError,
};
use std::fmt;

#[derive(Debug)]
pub enum DriverError {
    Parse(ParserError),
    Check(CheckError),
    Eval(EvalError),
    FileAccess(FileAccess),
    UndefinedLanguage(UndefinedLanguage),
    UndefinedFormatMethod(String),
    UndefinedCommand(String),
    EmptyInput,
    NoTyping(NoTyping),
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse(err) => err.fmt(f),
            Self::Check(err) => err.fmt(f),
            Self::Eval(err) => err.fmt(f),
            Self::UndefinedLanguage(ul) => ul.fmt(f),
            Self::UndefinedFormatMethod(method) => {
                write!(f, "Undefined Format Method {method}")
            }
            Self::UndefinedCommand(cmd) => write!(f, "{cmd} is not a valid command"),
            Self::EmptyInput => f.write_str("No source provided, use --input or --file"),
            Self::FileAccess(fa) => fa.fmt(f),
            Self::NoTyping(nt) => nt.fmt(f),
        }
    }
}

impl std::error::Error for DriverError {}

impl From<ParserError> for DriverError {
    fn from(err: ParserError) -> Self {
        Self::Parse(err)
    }
}

impl From<CheckError> for DriverError {
    fn from(err: CheckError) -> Self {
        Self::Check(err)
    }
}

impl From<EvalError> for DriverError {
    fn from(err: EvalError) -> Self {
        Self::Eval(err)
    }
}

impl From<FileAccess> for DriverError {
    fn from(err: FileAccess) -> Self {
        Self::FileAccess(err)
    }
}
impl From<UndefinedLanguage> for DriverError {
    fn from(err: UndefinedLanguage) -> Self {
        Self::UndefinedLanguage(err)
    }
}
impl From<NoTyping> for DriverError {
    fn from(err: NoTyping) -> Self {
        Self::NoTyping(err)
    }
}
