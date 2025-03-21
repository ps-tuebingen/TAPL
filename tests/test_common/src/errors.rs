use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    ReadDir(PathBuf),
    FileName(PathBuf),
    ReadFile(PathBuf),
    ParseToml(PathBuf),
    GetCurrentDir,
    SetCurrentDir,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadDir(path) => write!(f, "Could not read dir {path:?}"),
            Error::FileName(path) => write!(f, "Could not get file name for {path:?}"),
            Error::ReadFile(path) => write!(f, "COuld not read file {path:?}"),
            Error::ParseToml(path) => write!(f, "Could not parse toml file {path:?}"),
            Error::GetCurrentDir => f.write_str("Could not get current directory"),
            Error::SetCurrentDir => f.write_str("Could not set current directory"),
        }
    }
}

impl std::error::Error for Error {}
