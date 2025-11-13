use crate::{DirAccess, FileAccess, Toml};
use std::fmt;

#[derive(Debug)]
pub enum TestError {
    DirAccess(DirAccess),
    FileAccess(FileAccess),
    Toml(Toml),
}

impl TestError {}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DirAccess(da) => da.fmt(f),
            Self::FileAccess(fa) => fa.fmt(f),
            Self::Toml(t) => t.fmt(f),
        }
    }
}

impl std::error::Error for TestError {}

impl From<DirAccess> for TestError {
    fn from(da: DirAccess) -> Self {
        Self::DirAccess(da)
    }
}

impl From<FileAccess> for TestError {
    fn from(fa: FileAccess) -> Self {
        Self::FileAccess(fa)
    }
}

impl From<Toml> for TestError {
    fn from(t: Toml) -> Self {
        Self::Toml(t)
    }
}
