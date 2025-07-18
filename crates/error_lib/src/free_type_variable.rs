use std::fmt;

#[derive(Debug)]
pub struct FreeTypeVariable {
    var: String,
}

impl FreeTypeVariable {
    pub fn new(var: &String) -> FreeTypeVariable {
        FreeTypeVariable { var: var.clone() }
    }
}

impl fmt::Display for FreeTypeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Free Type Variable {}", self.var)
    }
}

impl std::error::Error for FreeTypeVariable {}
