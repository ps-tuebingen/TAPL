use std::fmt;

#[derive(Debug)]
pub struct FreeVariable {
    var: String,
}

impl FreeVariable {
    #[must_use] pub fn new(var: &str) -> Self {
        Self {
            var: var.to_owned(),
        }
    }
}

impl fmt::Display for FreeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Free Variable {}", self.var)
    }
}

impl std::error::Error for FreeVariable {}
