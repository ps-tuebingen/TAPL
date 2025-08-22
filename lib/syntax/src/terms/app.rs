use super::{Lambda, Term};
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
    types::Unit as UnitTy,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct App<Lang>
where
    Lang: Language,
{
    pub fun: Box<Lang::Term>,
    pub arg: Box<Lang::Term>,
}

impl<Lang> App<Lang>
where
    Lang: Language,
{
    pub fn new<F: Into<Lang::Term>, A: Into<Lang::Term>>(f: F, a: A) -> App<Lang> {
        App {
            fun: Box::new(f.into()),
            arg: Box::new(a.into()),
        }
    }

    pub fn seq<T1, T2, Ty>(t1: T1, t2: T2) -> App<Lang>
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        Lambda<Lang>: Into<Lang::Term>,
        UnitTy<Lang>: Into<Lang::Type>,
    {
        App {
            fun: Box::new(Lambda::new("_", UnitTy::new(), t2).into()),
            arg: Box::new(t1.into()),
        }
    }
}

impl<Lang> Term for App<Lang> where Lang: Language {}

impl<Lang> SubstTerm for App<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &Lang::Term) -> Self::Target {
        App {
            fun: Box::new(self.fun.subst(v, t)),
            arg: Box::new(self.arg.subst(v, t)),
        }
        .into()
    }
}
impl<Lang> SubstType for App<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        App {
            fun: Box::new(self.fun.subst_type(v, ty)),
            arg: Box::new(self.arg.subst_type(v, ty)),
        }
    }
}

impl<Lang> fmt::Display for App<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
