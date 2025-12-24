use crate::{FileAccess, check_error::CheckError, eval_error::EvalError, parse_error::ParserError};
use std::fmt;

#[derive(Debug)]
pub enum LanguageError {
    Parse(ParserError),
    Eval(EvalError),
    Check(CheckError),
    FileAccess(FileAccess),
}

impl fmt::Display for LanguageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse(err) => err.fmt(f),
            Self::Eval(err) => err.fmt(f),
            Self::Check(err) => err.fmt(f),
            Self::FileAccess(fa) => fa.fmt(f),
        }
    }
}

impl std::error::Error for LanguageError {}

impl From<ParserError> for LanguageError {
    fn from(err: ParserError) -> Self {
        Self::Parse(err)
    }
}

impl From<FileAccess> for LanguageError {
    fn from(err: FileAccess) -> Self {
        Self::FileAccess(err)
    }
}

impl From<EvalError> for LanguageError {
    fn from(err: EvalError) -> Self {
        Self::Eval(err)
    }
}

impl From<CheckError> for LanguageError {
    fn from(err: CheckError) -> Self {
        Self::Check(err)
    }
}
