use crate::KindVar;
use std::{fmt, rc::Rc};

/// Kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// Kind Variables (for inference)
    Var(KindVar),
    /// The star kind
    Star,
    /// An arrow kind
    Arrow(Rc<Kind>, Rc<Kind>),
}

impl Kind {
    /// Abstract `self`
    /// That is `self` -> * => `self`
    #[must_use]
    pub fn abs(self) -> Self {
        Self::Arrow(Rc::new(Self::Star), Rc::new(self))
    }

    /// Convert `self` to [`Kind::Star`]
    #[must_use]
    pub fn into_star(self) -> Option<Self> {
        if self == Self::Star { Some(self) } else { None }
    }

    /// Convert `self` into [`Kind::Arrow`]
    #[must_use]
    pub fn into_arrow(self) -> Option<(Self, Self)> {
        if let Self::Arrow(from, to) = self {
            Some((Rc::unwrap_or_clone(from), Rc::unwrap_or_clone(to)))
        } else {
            None
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Var(v) => write!(f, "{v}"),
            Self::Star => f.write_str("*"),
            Self::Arrow(from, to) => write!(f, "({from}) => ({to})"),
        }
    }
}

impl From<&str> for Kind {
    fn from(v: &str) -> Self {
        Self::Var(v.to_string())
    }
}

impl From<String> for Kind {
    fn from(s: String) -> Self {
        Self::Var(s)
    }
}
