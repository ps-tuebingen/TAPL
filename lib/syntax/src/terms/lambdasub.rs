use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
    types::Top,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LambdaSub<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub sup_ty: Lang::Type,
    pub body: Box<Lang::Term>,
}

impl<Lang> LambdaSub<Lang>
where
    Lang: Language,
{
    pub fn new<Ty, T>(v: &str, sup: Ty, bod: T) -> LambdaSub<Lang>
    where
        Ty: Into<Lang::Type>,
        T: Into<Lang::Term>,
    {
        LambdaSub {
            var: v.to_owned(),
            sup_ty: sup.into(),
            body: Box::new(bod.into()),
        }
    }

    pub fn new_unbounded<T>(v: &str, bod: T) -> LambdaSub<Lang>
    where
        T: Into<Lang::Term>,
        Top<Lang>: Into<Lang::Type>,
    {
        LambdaSub {
            var: v.to_owned(),
            sup_ty: Top::new_star().into(),
            body: Box::new(bod.into()),
        }
    }
}

impl<Lang> Term for LambdaSub<Lang> where Lang: Language {}

impl<Lang> SubstTerm for LambdaSub<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v == self.var {
            self
        } else {
            LambdaSub {
                var: self.var,
                sup_ty: self.sup_ty,
                body: Box::new(self.body.subst(v, t)),
            }
        }
    }
}

impl<Lang> SubstType for LambdaSub<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        let sup_subst = self.sup_ty.subst_type(v, ty);
        if *v == self.var {
            LambdaSub {
                var: self.var,
                sup_ty: sup_subst,
                body: self.body,
            }
        } else {
            LambdaSub {
                var: self.var,
                sup_ty: sup_subst,
                body: Box::new(self.body.subst_type(v, ty)),
            }
        }
    }
}

impl<Lang> fmt::Display for LambdaSub<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:({}).{}", self.var, self.sup_ty, self.body)
    }
}
