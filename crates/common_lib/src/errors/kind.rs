use pest::{error::Error as PestErr, RuleType};
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    FreeVariable(String),
    FreeTypeVariable(String),
    UndefinedLabel(String),
    UndefinedLocation(usize),
    UndefinedName(String),
    DefinedMultipleTimes(String),
    Subtype { sub: String, sup: String },
    NameMismatch { found: String, expected: String },
    TypeMismatch { found: String, expected: String },
    TermMismatch { found: String, expected: String },
    KindMismatch { found: String, expected: String },
    ValueMismatch { found: String, expected: String },
    Arity { found: usize, expected: usize },
    Infer { term: String, reason: String },
    // Parsing Errors
    Pest(String),
    MissingInput(String),
    RemainingInput(String),
    UnknownKeyword(String),
    UnexpectedRule { found: String, expected: String },
    UndefinedLanguage(String),
    DirAccess { tried: String, msg: String },
    Toml { source: String, msg: String },
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::FreeVariable(v) => write!(f, "Variable {v} appears free"),
            ErrorKind::FreeTypeVariable(v) => write!(f, "Type Variable {v} appears free"),
            ErrorKind::UndefinedLabel(lb) => write!(f, "Label {lb} was not defined"),
            ErrorKind::UndefinedName(name) => write!(f, "Name {name} was not defined"),
            ErrorKind::UndefinedLocation(loc) => write!(f, "Location {loc} was never defined"),
            ErrorKind::DefinedMultipleTimes(name) => write!(f, "{name} was defined multiple times"),
            ErrorKind::Subtype { sub, sup } => write!(f, "{sub} is not a subtype of {sup}"),
            ErrorKind::KindMismatch { found, expected } => {
                write!(f, "Kind Mismatch: found {found}, expected {expected}")
            }
            ErrorKind::NameMismatch { found, expected } => {
                write!(f, "Names {found} and {expected} should be equal")
            }
            ErrorKind::Arity { found, expected } => {
                write!(f, "Arity mismatch, expected {expected}, found {found}")
            }
            ErrorKind::Infer { term, reason } => {
                write!(f, "Could not infer type for {term}, {reason}")
            }
            ErrorKind::TypeMismatch { found, expected } => {
                write!(f, "Unexpected type {found}, expected {expected}")
            }
            ErrorKind::TermMismatch { found, expected } => {
                write!(f, "Unexpected term {found}, expected {expected}")
            }
            ErrorKind::ValueMismatch { found, expected } => {
                write!(f, "Unexpected value {found}, expected {expected}")
            }
            ErrorKind::Pest(msg) => write!(f, "Error in Pest: {msg}"),
            ErrorKind::MissingInput(missing) => write!(f, "Missing Input {missing}"),
            ErrorKind::RemainingInput(remaining) => write!(f, "Remaining Input {remaining}"),
            ErrorKind::UnknownKeyword(kw) => write!(f, "Unknown Keyword {kw}"),
            ErrorKind::UnexpectedRule { found, expected } => {
                write!(f, "Unexpeced rule {found}, expected {expected}")
            }
            ErrorKind::UndefinedLanguage(lang) => write!(f, "Language {lang} does not exist"),
            ErrorKind::DirAccess { tried, msg } => {
                write!(f, "Could not {tried}, error: {msg}")
            }
            ErrorKind::Toml { source, msg } => write!(f, "Could not parse toml: {msg}\n{source}"),
        }
    }
}

impl<T: RuleType> From<PestErr<T>> for ErrorKind {
    fn from(err: PestErr<T>) -> ErrorKind {
        ErrorKind::Pest(err.to_string())
    }
}
