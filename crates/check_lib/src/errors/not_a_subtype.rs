use std::fmt;
use syntax::types::Type;

#[derive(Debug)]
pub struct NotASubtype<Ty>
where
    Ty: Type,
{
    sub: Ty,
    sup: Ty,
}

impl<Ty> NotASubtype<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(sub: Ty1, sup: Ty2) -> NotASubtype<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        NotASubtype {
            sub: sub.into(),
            sup: sup.into(),
        }
    }
}

impl<Ty> fmt::Display for NotASubtype<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is not a subtype of {}", self.sub, self.sup)
    }
}

impl<Ty> std::error::Error for NotASubtype<Ty> where Ty: Type {}
