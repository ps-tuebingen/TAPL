use super::Value;
use crate::{Var, language::Language, terms::Lambda as LambdaT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lambda<Lang>
where
    Lang: Language,
{
    pub var: Var,
    pub annot: Lang::Type,
    pub body: Lang::Term,
}

impl<Lang> Lambda<Lang>
where
    Lang: Language,
{
    pub fn new<Ty, T>(v: &str, ty: Ty, bd: T) -> Lambda<Lang>
    where
        T: Into<Lang::Term>,
        Ty: Into<Lang::Type>,
        Lang: Language,
    {
        Lambda {
            var: v.to_owned(),
            annot: ty.into(),
            body: bd.into(),
        }
    }
}

impl<Lang> Value for Lambda<Lang>
where
    Lang: Language,
    LambdaT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = LambdaT<Lang>;
}

impl<Lang> From<Lambda<Lang>> for LambdaT<Lang>
where
    Lang: Language,
{
    fn from(lam: Lambda<Lang>) -> LambdaT<Lang> {
        LambdaT::new(&lam.var, lam.annot, lam.body)
    }
}

impl<Lang> fmt::Display for Lambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty_str = self.annot.to_string();
        if ty_str.is_empty() {
            write!(f, "\\{}.{}", self.var, self.body)
        } else {
            write!(f, "\\{}:{}.({})", self.var, ty_str, self.body)
        }
    }
}
