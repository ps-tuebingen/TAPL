use std::fmt;
use syntax::types::Type;

#[derive(Debug)]
pub struct NotASubtype<Ty1, Ty2>
where
    Ty1: Type,
    Ty2: Type,
{
    sub: Ty1,
    sup: Ty2,
}

impl<Ty1, Ty2> NotASubtype<Ty1, Ty2>
where
    Ty1: Type,
    Ty2: Type,
{
    pub fn new(sub: Ty1, sup: Ty2) -> NotASubtype<Ty1, Ty2> {
        NotASubtype { sub, sup }
    }
}

impl<Ty1, Ty2> fmt::Display for NotASubtype<Ty1, Ty2>
where
    Ty1: Type,
    Ty2: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is not a subtype of {}", self.sub, self.sup)
    }
}

impl<Ty1, Ty2> std::error::Error for NotASubtype<Ty1, Ty2>
where
    Ty1: Type,
    Ty2: Type,
{
}
