use std::fmt;

#[derive(Debug)]
pub struct DirAccess {
    tried: String,
    msg: String,
}

impl DirAccess {
    pub fn new<E>(tried: &str, err: E) -> DirAccess
    where
        E: std::error::Error,
    {
        DirAccess {
            tried: tried.to_owned(),
            msg: err.to_string(),
        }
    }
}

impl fmt::Display for DirAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error accessing directory, tried to {}\n\t{}",
            self.tried, self.msg
        )
    }
}

impl std::error::Error for DirAccess {}
