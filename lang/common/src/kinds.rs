use crate::errors::ErrorKind;
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

    pub fn into_star(self) -> Result<Kind, ErrorKind> {
        if let Kind::Star = self {
            Ok(self)
        } else {
            Err(ErrorKind::KindMismatch {
                found: self.to_string(),
                expected: "*".to_owned(),
            })
        }
    }

    pub fn into_arrow(self) -> Result<(Kind, Kind), ErrorKind> {
        if let Kind::Arrow(from, to) = self {
            Ok((*from, *to))
        } else {
            Err(ErrorKind::KindMismatch {
                found: self.to_string(),
                expected: "Arrow Kind".to_owned(),
            })
        }
    }

    pub fn check_equal(&self, other: &Kind) -> Result<(), ErrorKind> {
        if *self == *other {
            Ok(())
        } else {
            Err(ErrorKind::KindMismatch {
                found: other.to_string(),
                expected: self.to_string(),
            })
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Star => f.write_str("*"),
            Kind::Arrow(from, to) => write!(f, "({from}) => ({to}"),
        }
    }
}
