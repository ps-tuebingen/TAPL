use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mu<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub ty: Box<Lang::Type>,
}

impl<Lang> Mu<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(v: &str, ty: Ty1) -> Mu<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        Mu {
            var: v.to_owned(),
            ty: Box::new(ty.into()),
        }
    }
}

impl<Lang> Type for Mu<Lang> where Lang: Language {}

impl<Lang> SubstType for Mu<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        if *v == self.var {
            self
        } else {
            Mu {
                var: self.var,
                ty: Box::new(self.ty.subst_type(v, ty)),
            }
        }
    }
}

impl<Lang> fmt::Display for Mu<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mu {}.{}", self.var, self.ty)
    }
}
