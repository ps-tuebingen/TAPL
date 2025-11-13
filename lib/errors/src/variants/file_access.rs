use std::fmt;

#[derive(Debug)]
pub struct FileAccess {
    tried: String,
    msg: String,
}

impl FileAccess {
    pub fn new<T>(tried: &str, t: T) -> Self
    where
        T: fmt::Display,
    {
        Self {
            tried: tried.to_owned(),
            msg: t.to_string(),
        }
    }
}

impl fmt::Display for FileAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error accessing file, tried to {}\n\t{}",
            self.tried, self.msg
        )
    }
}
