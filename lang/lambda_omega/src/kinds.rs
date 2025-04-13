use common::errors::ErrorKind;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Star,
    Arrow(Box<Kind>, Box<Kind>),
}

impl Kind {
    pub fn as_arrow(self) -> Result<(Kind, Kind), ErrorKind> {
        if let Kind::Arrow(left, right) = self {
            Ok((*left, *right))
        } else {
            Err(ErrorKind::KindMismatch {
                found: self.to_string(),
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
