use super::{ExistsBounded, Top, Type};
use crate::{TypeVar, kinds::Kind, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exists<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub kind: Kind,
    pub ty: Rc<Lang::Type>,
}

impl<Lang> Exists<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(v: &str, knd: Kind, ty: Ty1) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            kind: knd,
            ty: Rc::new(ty.into()),
        }
    }

    #[must_use] pub fn to_exists_bounded(self) -> ExistsBounded<Lang>
    where
        Top<Lang>: Into<Lang::Type>,
    {
        ExistsBounded::new_unbounded(&self.var, self.kind, Rc::unwrap_or_clone(self.ty))
    }
}

impl<Lang> Type for Exists<Lang> where Lang: Language {}

impl<Lang> SubstType for Exists<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;

    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        if *v == self.var {
            self
        } else {
            Self {
                var: self.var,
                kind: self.kind,
                ty: self.ty.subst_type(v, ty),
            }
        }
    }
}

impl<Lang> fmt::Display for Exists<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{exists {}::{},{}}}", self.var, self.kind, self.ty)
    }
}
