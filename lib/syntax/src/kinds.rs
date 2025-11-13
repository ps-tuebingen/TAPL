use errors::KindMismatch;
use std::{fmt, rc::Rc};

/// Kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
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
    /// # Errors
    /// Returns an error if `self` is an arrow kind
    pub fn into_star(self) -> Result<Self, KindMismatch> {
        if self == Self::Star {
            Ok(self)
        } else {
            Err(KindMismatch::new(self.to_string(), Self::Star.to_string()))
        }
    }

    /// Convert `self` into [`Kind::Arrow`]
    /// # Errors
    /// returns an error if `self` is a star kind
    pub fn into_arrow(self) -> Result<(Self, Self), KindMismatch> {
        if let Self::Arrow(from, to) = self {
            Ok((Rc::unwrap_or_clone(from), Rc::unwrap_or_clone(to)))
        } else {
            Err(KindMismatch::new(
                self.to_string(),
                Self::Arrow(Rc::new(Self::Star), Rc::new(Self::Star)).to_string(),
            ))
        }
    }

    /// Assert `self` and `other` are equal
    /// # Errors
    /// Returns an error if `self != other`
    pub fn check_equal(&self, other: &Self) -> Result<(), KindMismatch> {
        if *self == *other {
            Ok(())
        } else {
            Err(KindMismatch::new(
                other.clone().to_string(),
                self.clone().to_string(),
            ))
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Star => f.write_str("*"),
            Self::Arrow(from, to) => write!(f, "({from}) => ({to})"),
        }
    }
}
