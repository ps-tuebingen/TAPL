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
            BuildError::DirAccess(da) => da.fmt(f),
            BuildError::FileAccess(fa) => fa.fmt(f),
            BuildError::UndefinedLanguage(ul) => ul.fmt(f),
        }
    }
}

impl std::error::Error for BuildError {}

impl From<DirAccess> for BuildError {
    fn from(err: DirAccess) -> BuildError {
        BuildError::DirAccess(err)
    }
}

impl From<FileAccess> for BuildError {
    fn from(err: FileAccess) -> BuildError {
        BuildError::FileAccess(err)
    }
}

impl From<UndefinedLanguage> for BuildError {
    fn from(err: UndefinedLanguage) -> BuildError {
        BuildError::UndefinedLanguage(err)
    }
}
