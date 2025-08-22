use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lambda<Lang>
where
    Lang: Language,
{
    pub var: Var,
    pub annot: Lang::Type,
    pub body: Box<Lang::Term>,
}

impl<Lang> Lambda<Lang>
where
    Lang: Language,
{
    pub fn new<T, Ty>(v: &str, ty: Ty, t: T) -> Lambda<Lang>
    where
        T: Into<Lang::Term>,
        Ty: Into<Lang::Type>,
    {
        Lambda {
            var: v.to_owned(),
            annot: ty.into(),
            body: Box::new(t.into()),
        }
    }
}

impl<Lang> Term for Lambda<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Lambda<Lang>
where
    Lang: Language,
    Self: Into<Lang::Term>,
{
    type Target = Self;
    type Lang = Lang;

    fn subst(self, v: &Var, t: &<Self::Lang as Language>::Term) -> Self::Target {
        if *v == self.var {
            self
        } else {
            Lambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst(v, t)),
            }
        }
    }
}

impl<Lang> SubstType for Lambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Lambda {
            var: self.var,
            annot: self.annot.subst_type(v, ty),
            body: Box::new(self.body.subst_type(v, ty)),
        }
        .into()
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
