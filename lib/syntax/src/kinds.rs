use errors::KindMismatch;
use std::{fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Star,
    Arrow(Rc<Kind>, Rc<Kind>),
}

impl Kind {
    #[must_use] pub fn abs(self) -> Self {
        Self::Arrow(Rc::new(Self::Star), Rc::new(self))
    }

    pub fn into_star(self) -> Result<Self, KindMismatch> {
        if self == Self::Star {
            Ok(self)
        } else {
            Err(KindMismatch::new(self.to_string(), Self::Star.to_string()))
        }
    }

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
