use std::fmt;

/// Error during file access
#[derive(Debug)]
pub struct FileAccess {
    /// action that was tried
    tried: String,
    /// Error message
    msg: String,
}

impl FileAccess {
    /// Create a new error from tried action and error
    /// usually [`std::io::Error`]
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
