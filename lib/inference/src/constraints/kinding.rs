use super::Constraint;
use syntax::{kinds::Kind, language::Language};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KindOrVar {
    Kind(Kind),
    Var(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindConstraint {
    left: KindOrVar,
    right: KindOrVar,
}

impl KindConstraint {
    pub fn new<K1, K2>(left: K1, right: K2) -> Self
    where
        K1: Into<KindOrVar>,
        K2: Into<KindOrVar>,
    {
        Self {
            left: left.into(),
            right: right.into(),
        }
    }
}

impl From<Kind> for KindOrVar {
    fn from(knd: Kind) -> Self {
        Self::Kind(knd)
    }
}

impl From<&str> for KindOrVar {
    fn from(v: &str) -> Self {
        Self::Var(v.to_string())
    }
}

impl<Lang> From<KindConstraint> for Constraint<Lang>
where
    Lang: Language,
{
    fn from(c: KindConstraint) -> Self {
        Constraint::Kinding(c)
    }
}
