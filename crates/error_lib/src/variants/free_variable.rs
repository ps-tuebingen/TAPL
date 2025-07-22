use std::fmt;

#[derive(Debug)]
pub struct FreeVariable {
    var: String,
}

impl FreeVariable {
    pub fn new(var: &String) -> FreeVariable {
        FreeVariable { var: var.clone() }
    }
}

impl fmt::Display for FreeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Free Variable {}", self.var)
    }
}

impl std::error::Error for FreeVariable {}
