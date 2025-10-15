use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UntypedLambda<Lang>
where
    Lang: Language,
{
    pub var: Var,
    pub body: Box<Lang::Term>,
}

impl<Lang> UntypedLambda<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(v: &str, t: T1) -> UntypedLambda<Lang>
    where
        T1: Into<Lang::Term>,
    {
        UntypedLambda {
            var: v.to_owned(),
            body: Box::new(t.into()),
        }
    }
}

impl<Lang> Term for UntypedLambda<Lang> where Lang: Language {}

impl<Lang> SubstTerm for UntypedLambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v == self.var {
            self
        } else {
            UntypedLambda {
                var: self.var,
                body: Box::new(self.body.subst(v, t)),
            }
        }
    }
}

impl<Lang> SubstType for UntypedLambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        UntypedLambda {
            var: self.var,
            body: Box::new(self.body.subst_type(v, ty)),
        }
    }
}

impl<Lang> fmt::Display for UntypedLambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}.{}", self.var, self.body)
    }
}
