use crate::{
    FileAccess, UndefinedLanguage, check_error::CheckError, eval_error::EvalError,
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
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DriverError::Parse(err) => err.fmt(f),
            DriverError::Check(err) => err.fmt(f),
            DriverError::Eval(err) => err.fmt(f),
            DriverError::UndefinedLanguage(ul) => ul.fmt(f),
            DriverError::UndefinedFormatMethod(method) => {
                write!(f, "Undefined Format Method {method}")
            }
            DriverError::UndefinedCommand(cmd) => write!(f, "{cmd} is not a valid command"),
            DriverError::EmptyInput => f.write_str("No source provided, use --input or --file"),
            DriverError::FileAccess(fa) => fa.fmt(f),
        }
    }
}

impl std::error::Error for DriverError {}

impl From<ParserError> for DriverError {
    fn from(err: ParserError) -> DriverError {
        DriverError::Parse(err)
    }
}

impl From<CheckError> for DriverError {
    fn from(err: CheckError) -> DriverError {
        DriverError::Check(err)
    }
}

impl From<EvalError> for DriverError {
    fn from(err: EvalError) -> DriverError {
        DriverError::Eval(err)
    }
}

impl From<FileAccess> for DriverError {
    fn from(err: FileAccess) -> DriverError {
        DriverError::FileAccess(err)
    }
}
impl From<UndefinedLanguage> for DriverError {
    fn from(err: UndefinedLanguage) -> DriverError {
        DriverError::UndefinedLanguage(err)
    }
}
