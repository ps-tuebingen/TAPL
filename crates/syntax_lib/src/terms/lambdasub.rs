use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Top,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LambdaSub<T>
where
    T: Term,
{
    var: TypeVar,
    sup_ty: <T as Term>::Type,
    body: Box<T>,
}

impl<T> LambdaSub<T>
where
    T: Term,
{
    pub fn new<Typ, B>(v: &str, sup: Typ, bod: B) -> LambdaSub<T>
    where
        Typ: Into<<T as Term>::Type>,
        B: Into<T>,
    {
        LambdaSub {
            var: v.to_owned(),
            sup_ty: sup.into(),
            body: Box::new(bod.into()),
        }
    }

    pub fn new_unbounded<T1>(v: &str, bod: T1) -> LambdaSub<T>
    where
        T1: Into<T>,
        Top<<T as Term>::Type>: Into<<T as LanguageTerm>::Type>,
    {
        LambdaSub {
            var: v.to_owned(),
            sup_ty: Top::new_star().into(),
            body: Box::new(bod.into()),
        }
    }
}

impl<T> Term for LambdaSub<T> where T: Term {}

impl<T> SubstTerm<T> for LambdaSub<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            LambdaSub {
                var: self.var,
                sup_ty: self.sup_ty,
                body: Box::new(self.body.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as Term>::Type> for LambdaSub<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        let sup_subst = self.sup_ty.subst_type(v, ty);
        if *v == self.var {
            LambdaSub {
                var: self.var,
                sup_ty: sup_subst,
                body: self.body,
            }
            .into()
        } else {
            LambdaSub {
                var: self.var,
                sup_ty: sup_subst,
                body: Box::new(self.body.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<T> fmt::Display for LambdaSub<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:({}).{}", self.var, self.sup_ty, self.body)
    }
}
