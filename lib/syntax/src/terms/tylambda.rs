use super::Term;
use crate::{
    TypeVar, Var,
    kinds::Kind,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyLambda<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub annot: Kind,
    pub term: Rc<Lang::Term>,
}

impl<Lang> TyLambda<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(v: &str, knd: Kind, t: T1) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            var: v.into(),
            annot: knd,
            term: Rc::new(t.into()),
        }
    }
}

impl<Lang> Term for TyLambda<Lang> where Lang: Language {}

impl<Lang> SubstTerm for TyLambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            var: self.var,
            annot: self.annot,
            term: self.term.subst(v, t),
        }
    }
}

impl<Lang> SubstType for TyLambda<Lang>
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
                annot: self.annot,
                term: self.term.subst_type(v, ty),
            }
        }
    }
}

impl<Lang> fmt::Display for TyLambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.({})", self.var, self.annot, self.term)
    }
}
