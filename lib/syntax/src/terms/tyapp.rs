use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyApp<Lang>
where
    Lang: Language,
{
    pub fun: Rc<Lang::Term>,
    pub arg: Lang::Type,
}

impl<Lang> TyApp<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Typ>(t: T1, ty: Typ) -> Self
    where
        T1: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Self {
            fun: Rc::new(t.into()),
            arg: ty.into(),
        }
    }
}

impl<Lang> Term for TyApp<Lang> where Lang: Language {}

impl<Lang> SubstTerm for TyApp<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            fun: self.fun.subst(v, t),
            arg: self.arg,
        }
    }
}

impl<Lang> SubstType for TyApp<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            fun: self.fun.subst_type(v, ty),
            arg: self.arg.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for TyApp<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(({})[{}])", self.fun, self.arg)
    }
}
