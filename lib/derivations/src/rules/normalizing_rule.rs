use std::fmt;

#[derive(Debug)]
pub enum NormalizingRule {
    Refl,
    Cong,
    OpApp,
}

impl fmt::Display for NormalizingRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Refl => f.write_str("Norm-Refl"),
            Self::Cong => f.write_str("Norm-Cong"),
            Self::OpApp => f.write_str("Norm-AppAbs"),
        }
    }
}
