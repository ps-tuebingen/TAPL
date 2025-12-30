use crate::{
    FileAccess, UndefinedLanguage, check_error::CheckError, eval_error::EvalError,
    inference_error::InferenceError, parse_error::ParserError,
};
use std::fmt;

#[derive(Debug)]
pub enum LanguageError {
    Parse(ParserError),
    Eval(EvalError),
    Check(CheckError),
    Inference(InferenceError),
    FileAccess(FileAccess),
    UndefinedLanguage(UndefinedLanguage),
    UndefinedCommand(String),
}

impl fmt::Display for LanguageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse(err) => err.fmt(f),
            Self::Eval(err) => err.fmt(f),
            Self::Check(err) => err.fmt(f),
            Self::FileAccess(fa) => fa.fmt(f),
            Self::UndefinedLanguage(ua) => ua.fmt(f),
            Self::UndefinedCommand(cmd) => write!(f, "{cmd} is not a valid command"),
            Self::Inference(err) => err.fmt(f),
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

impl From<UndefinedLanguage> for LanguageError {
    fn from(err: UndefinedLanguage) -> Self {
        Self::UndefinedLanguage(err)
    }
}

impl From<InferenceError> for LanguageError {
    fn from(err: InferenceError) -> Self {
        Self::Inference(err)
    }
}
