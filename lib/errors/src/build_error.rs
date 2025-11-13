use crate::{DirAccess, FileAccess, UndefinedLanguage};
use std::fmt;

#[derive(Debug)]
pub enum BuildError {
    DirAccess(DirAccess),
    FileAccess(FileAccess),
    UndefinedLanguage(UndefinedLanguage),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DirAccess(da) => da.fmt(f),
            Self::FileAccess(fa) => fa.fmt(f),
            Self::UndefinedLanguage(ul) => ul.fmt(f),
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

impl From<UndefinedLanguage> for BuildError {
    fn from(err: UndefinedLanguage) -> Self {
        Self::UndefinedLanguage(err)
    }
}
