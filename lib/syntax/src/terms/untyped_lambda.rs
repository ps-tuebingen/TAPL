use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UntypedLambda<Lang>
where
    Lang: Language,
{
    pub var: Var,
    pub body: Rc<Lang::Term>,
}

impl<Lang> UntypedLambda<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(v: &str, t: T1) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            body: Rc::new(t.into()),
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
            Self {
                var: self.var,
                body: self.body.subst(v, t),
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
        Self {
            var: self.var,
            body: self.body.subst_type(v, ty),
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
