use check::errors::CheckError;
use eval::errors::EvalError;
use parse::errors::ParserError;
use std::fmt;

#[derive(Debug)]
pub enum LanguageError {
    Parse(ParserError),
    Check(CheckError),
    Eval(EvalError),
    UndefinedLanguage(String),
}

impl fmt::Display for LanguageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LanguageError::Parse(err) => err.fmt(f),
            LanguageError::Check(err) => err.fmt(f),
            LanguageError::Eval(err) => err.fmt(f),
            LanguageError::UndefinedLanguage(lang) => write!(f, "Undefined Language {lang}"),
        }
    }
}

impl std::error::Error for LanguageError {}

impl From<ParserError> for LanguageError {
    fn from(err: ParserError) -> LanguageError {
        LanguageError::Parse(err)
    }
}

impl From<CheckError> for LanguageError {
    fn from(err: CheckError) -> LanguageError {
        LanguageError::Check(err)
    }
}

impl From<EvalError> for LanguageError {
    fn from(err: EvalError) -> LanguageError {
        LanguageError::Eval(err)
    }
}
