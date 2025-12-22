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
