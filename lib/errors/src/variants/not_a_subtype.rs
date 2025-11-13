use std::fmt;

#[derive(Debug)]
pub struct NotASubtype {
    sub: String,
    sup: String,
}

impl NotASubtype {
    pub fn new<Ty1, Ty2>(sub_ty: Ty1, super_ty: Ty2) -> Self
    where
        Ty1: fmt::Display,
        Ty2: fmt::Display,
    {
        Self {
            sub: sub_ty.to_string(),
            sup: super_ty.to_string(),
        }
    }
}

impl fmt::Display for NotASubtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is not a subtype of {}", self.sub, self.sup)
    }
}

impl std::error::Error for NotASubtype {}
