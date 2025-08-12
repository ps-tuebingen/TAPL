use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If<T>
where
    T: Term,
{
    pub if_cond: Box<T>,
    pub then_term: Box<T>,
    pub else_term: Box<T>,
}

impl<T> If<T>
where
    T: Term,
{
    pub fn new<T1, T2, T3>(cond: T1, th: T2, els: T3) -> If<T>
    where
        T1: Into<T>,
        T2: Into<T>,
        T3: Into<T>,
    {
        If {
            if_cond: Box::new(cond.into()),
            then_term: Box::new(th.into()),
            else_term: Box::new(els.into()),
        }
    }
}

impl<T> Term for If<T> where T: Term {}

impl<T> SubstTerm<T> for If<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        If {
            if_cond: Box::new(self.if_cond.subst(v, t)),
            then_term: Box::new(self.then_term.subst(v, t)),
            else_term: Box::new(self.else_term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for If<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        If {
            if_cond: Box::new(self.if_cond.subst_type(v, ty)),
            then_term: Box::new(self.then_term.subst_type(v, ty)),
            else_term: Box::new(self.else_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for If<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.if_cond, self.then_term, self.else_term
        )
    }
}
