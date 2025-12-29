use crate::{DirAccess, FileAccess, UndefinedLanguage, language_error::LanguageError};
use std::fmt;

#[derive(Debug)]
pub enum BuildError {
    DirAccess(DirAccess),
    FileAccess(FileAccess),
    Language(LanguageError),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DirAccess(da) => da.fmt(f),
            Self::FileAccess(fa) => fa.fmt(f),
            Self::Language(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for BuildError {}

impl From<DirAccess> for BuildError {
    fn from(err: DirAccess) -> Self {
        Self::DirAccess(err)
    }
}

impl From<FileAccess> for BuildError {
    fn from(err: FileAccess) -> Self {
        Self::FileAccess(err)
    }
}

impl From<LanguageError> for BuildError {
    fn from(err: LanguageError) -> Self {
        Self::Language(err)
    }
}
