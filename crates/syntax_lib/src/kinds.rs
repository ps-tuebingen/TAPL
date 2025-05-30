use super::errors::{Error, KindKind};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Star,
    Arrow(Box<Kind>, Box<Kind>),
}

impl Kind {
    pub fn abs(self) -> Kind {
        Kind::Arrow(Box::new(Kind::Star), Box::new(self))
    }

    pub fn into_star(self) -> Result<Kind, Error> {
        if let Kind::Star = self {
            Ok(self)
        } else {
            Err(Error::kind(self, KindKind::Star))
        }
    }

    pub fn into_arrow(self) -> Result<(Kind, Kind), Error> {
        if let Kind::Arrow(from, to) = self {
            Ok((*from, *to))
        } else {
            Err(Error::kind(self, KindKind::Arrow))
        }
    }

    pub fn check_equal(&self, other: &Kind) -> Result<(), Error> {
        if *self == *other {
            Ok(())
        } else {
            Err(Error::kind(other.clone(), self.clone().into()))
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Star => f.write_str("*"),
            Kind::Arrow(from, to) => write!(f, "({from}) => ({to})"),
        }
    }
}
