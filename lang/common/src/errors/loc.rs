use std::fmt;

#[derive(Debug)]
pub enum ErrorLocation {
    Eval,
    Check,
    Subtyping,
    Parse,
    Kind,
    Inference,
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorLocation::Eval => f.write_str("Evaluation"),
            ErrorLocation::Check => f.write_str("Checking"),
            ErrorLocation::Subtyping => f.write_str("Subtyping"),
            ErrorLocation::Parse => f.write_str("Parsing"),
            ErrorLocation::Kind => f.write_str("Kinding"),
            ErrorLocation::Inference => f.write_str("Inference"),
        }
    }
}
