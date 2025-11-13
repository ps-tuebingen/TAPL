use std::fmt;

#[derive(Debug)]
pub struct FreeTypeVariable {
    var: String,
}

impl FreeTypeVariable {
    #[must_use] pub fn new(var: &str) -> Self {
        Self {
            var: var.to_owned(),
        }
    }
}

impl fmt::Display for FreeTypeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Free Type Variable {}", self.var)
    }
}

impl std::error::Error for FreeTypeVariable {}
