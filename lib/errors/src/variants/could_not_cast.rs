use std::fmt;

#[derive(Debug)]
pub struct CouldNotCast {
    id: String,
    target: String,
}

impl CouldNotCast {
    pub fn new(id: &str, target: &str) -> CouldNotCast {
        CouldNotCast {
            id: id.to_owned(),
            target: target.to_owned(),
        }
    }
}

impl fmt::Display for CouldNotCast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not cast element {} to {}", self.id, self.target)
    }
}

impl std::error::Error for CouldNotCast {}
