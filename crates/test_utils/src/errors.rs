use std::fmt;

#[derive(Debug)]
pub enum Error {
    DirAccess { tried: String, msg: String },
    FileAccess { tried: String, msg: String },
    Toml { source: String, msg: String },
}

impl Error {
    pub fn dir_access<E>(tried: &str, err: E) -> Error
    where
        E: std::error::Error,
    {
        Error::DirAccess {
            tried: tried.to_owned(),
            msg: err.to_string(),
        }
    }

    pub fn file_access<T>(tried: &str, t: T) -> Error
    where
        T: fmt::Display,
    {
        Error::FileAccess {
            tried: tried.to_owned(),
            msg: t.to_string(),
        }
    }

    pub fn toml<E>(src: &str, err: E) -> Error
    where
        E: std::error::Error,
    {
        Error::Toml {
            source: src.to_owned(),
            msg: err.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DirAccess { tried, msg } => {
                write!(f, "Error accessing directory, tried to {tried}\n\t{msg}")
            }
            Error::FileAccess { tried, msg } => {
                write!(f, "Error accessing file, tried to {tried}\n\t{msg}")
            }
            Error::Toml { source, msg } => write!(f, "Could not parse toml: {msg}\n\t{source}"),
        }
    }
}

impl std::error::Error for Error {}
