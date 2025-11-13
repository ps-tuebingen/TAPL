use super::Value;
use crate::{TypeVar, kinds::Kind, language::Language, terms::TyLambda as TyLambdaT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TyLambda<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub annot: Kind,
    pub term: Lang::Term,
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
            var: v.to_owned(),
            annot: knd,
            term: t.into(),
        }
    }
}

impl<Lang> Value for TyLambda<Lang>
where
    Lang: Language,
    TyLambdaT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = TyLambdaT<Lang>;
}
impl<Lang> From<TyLambda<Lang>> for TyLambdaT<Lang>
where
    Lang: Language,
{
    fn from(tylam: TyLambda<Lang>) -> Self {
        Self::new(&tylam.var, tylam.annot, tylam.term)
    }
}

impl<Lang> fmt::Display for TyLambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.{}", self.var, self.annot, self.term)
    }
}
