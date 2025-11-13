use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
    types::Top,
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LambdaSub<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub sup_ty: Lang::Type,
    pub body: Rc<Lang::Term>,
}

impl<Lang> LambdaSub<Lang>
where
    Lang: Language,
{
    pub fn new<Ty, T>(v: &str, sup: Ty, bod: T) -> Self
    where
        Ty: Into<Lang::Type>,
        T: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: sup.into(),
            body: Rc::new(bod.into()),
        }
    }

    pub fn new_unbounded<T>(v: &str, bod: T) -> Self
    where
        T: Into<Lang::Term>,
        Top<Lang>: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: Top::new_star().into(),
            body: Rc::new(bod.into()),
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
            Self {
                var: self.var,
                sup_ty: self.sup_ty,
                body: self.body.subst(v, t),
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
            Self {
                var: self.var,
                sup_ty: sup_subst,
                body: self.body,
            }
        } else {
            Self {
                var: self.var,
                sup_ty: sup_subst,
                body: self.body.subst_type(v, ty),
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
