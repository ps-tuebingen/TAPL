use common::errors::{KindKind, KindMismatch};
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

    pub fn into_star(self) -> Result<Kind, KindMismatch> {
        if let Kind::Star = self {
            Ok(self)
        } else {
            Err(KindMismatch::new(self.into(), KindKind::Star))
        }
    }

    pub fn into_arrow(self) -> Result<(Kind, Kind), KindMismatch> {
        if let Kind::Arrow(from, to) = self {
            Ok((*from, *to))
        } else {
            Err(KindMismatch::new(self.into(), KindKind::Arrow))
        }
    }

    pub fn check_equal(&self, other: &Kind) -> Result<(), KindMismatch> {
        if *self == *other {
            Ok(())
        } else {
            Err(KindMismatch::new(other.clone().into(), self.clone().into()))
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

impl From<Kind> for KindKind {
    fn from(knd: Kind) -> KindKind {
        match knd {
            Kind::Star => KindKind::Star,
            Kind::Arrow(_, _) => KindKind::Arrow,
        }
    }
}
