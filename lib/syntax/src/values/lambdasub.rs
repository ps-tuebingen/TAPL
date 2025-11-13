use super::Value;
use crate::{Var, language::Language, terms::LambdaSub as LambdaSubT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LambdaSub<Lang>
where
    Lang: Language,
{
    pub var: Var,
    pub sup_ty: Lang::Type,
    pub term: Lang::Term,
}

impl<Lang> LambdaSub<Lang>
where
    Lang: Language,
{
    pub fn new<Ty, T>(v: &str, sup: Ty, t: T) -> Self
    where
        Ty: Into<Lang::Type>,
        T: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: sup.into(),
            term: t.into(),
        }
    }
}

impl<Lang> Value for LambdaSub<Lang>
where
    Lang: Language,
    LambdaSubT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = LambdaSubT<Lang>;
}

impl<Lang> From<LambdaSub<Lang>> for LambdaSubT<Lang>
where
    Lang: Language,
{
    fn from(lam: LambdaSub<Lang>) -> Self {
        Self::new(&lam.var, lam.sup_ty, lam.term)
    }
}

impl<Lang> fmt::Display for LambdaSub<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:({}).{}", self.var, self.sup_ty, self.term)
    }
}
