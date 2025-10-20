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
            NormalizingRule::Refl => f.write_str("Norm-Refl"),
            NormalizingRule::Cong => f.write_str("Norm-Cong"),
            NormalizingRule::OpApp => f.write_str("Norm-AppAbs"),
        }
    }
}
