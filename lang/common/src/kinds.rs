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
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Star => f.write_str("*"),
            Kind::Arrow(from, to) => write!(f, "({from}) => ({to}"),
        }
    }
}
