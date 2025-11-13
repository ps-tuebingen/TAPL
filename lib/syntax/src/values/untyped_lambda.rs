use super::Value;
use crate::{Var, language::Language, terms::UntypedLambda as UntypedLambdaT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UntypedLambda<Lang>
where
    Lang: Language,
{
    pub var: Var,
    pub body: Lang::Term,
}

impl<Lang> UntypedLambda<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(v: &str, bd: T1) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            body: bd.into(),
        }
    }
}

impl<Lang> Value for UntypedLambda<Lang>
where
    Lang: Language,
    UntypedLambdaT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = UntypedLambdaT<Lang>;
}

impl<Lang> From<UntypedLambda<Lang>> for UntypedLambdaT<Lang>
where
    Lang: Language,
{
    fn from(lam: UntypedLambda<Lang>) -> Self {
        Self::new(&lam.var, lam.body)
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
