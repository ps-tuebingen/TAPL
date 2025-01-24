use crate::errors::Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Star,
    Arrow(Box<Kind>, Box<Kind>),
}

impl Kind {
    pub fn as_arrow(self) -> Result<(Kind, Kind), Error> {
        if let Kind::Arrow(left, right) = self {
            Ok((*left, *right))
        } else {
            Err(Error::KindMismatch {
                found: self,
                expected: "Arrow Kind".to_owned(),
            })
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Star => f.write_str("*"),
            Kind::Arrow(left, right) => write!(f, "{left}⇒{right}"),
        }
    }
}
