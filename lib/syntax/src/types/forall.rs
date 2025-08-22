use super::Type;
use crate::{TypeVar, kinds::Kind, language::Language, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub kind: Kind,
    pub ty: Box<Lang::Type>,
}

impl<Lang> Forall<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(v: &str, knd: Kind, ty: Ty1) -> Forall<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        Forall {
            var: v.to_owned(),
            kind: knd,
            ty: Box::new(ty.into()),
        }
    }
}

impl<Lang> Type for Forall<Lang> where Lang: Language {}

impl<Lang> SubstType for Forall<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        if *v == self.var {
            self
        } else {
            Forall {
                var: self.var,
                kind: self.kind,
                ty: Box::new(self.ty.subst_type(v, ty)),
            }
        }
    }
}

impl<Lang> fmt::Display for Forall<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}::{}.{}", self.var, self.kind, self.ty)
    }
}
