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
            TestError::DirAccess(da) => da.fmt(f),
            TestError::FileAccess(fa) => fa.fmt(f),
            TestError::Toml(t) => t.fmt(f),
        }
    }
}

impl std::error::Error for TestError {}

impl From<DirAccess> for TestError {
    fn from(da: DirAccess) -> TestError {
        TestError::DirAccess(da)
    }
}

impl From<FileAccess> for TestError {
    fn from(fa: FileAccess) -> TestError {
        TestError::FileAccess(fa)
    }
}

impl From<Toml> for TestError {
    fn from(t: Toml) -> TestError {
        TestError::Toml(t)
    }
}
