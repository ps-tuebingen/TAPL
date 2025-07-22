use std::fmt;

#[derive(Debug)]
pub struct NotASubtype {
    sub: String,
    sup: String,
}

impl NotASubtype {
    pub fn new<Ty1, Ty2>(sub: Ty1, sup: Ty2) -> NotASubtype
    where
        Ty1: fmt::Display,
        Ty2: fmt::Display,
    {
        NotASubtype {
            sub: sub.to_string(),
            sup: sup.to_string(),
        }
    }
}

impl fmt::Display for NotASubtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is not a subtype of {}", self.sub, self.sup)
    }
}

impl std::error::Error for NotASubtype {}
