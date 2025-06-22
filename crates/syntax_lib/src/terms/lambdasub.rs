use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Top,
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub var: TypeVar,
    pub sup_ty: Ty,
    pub body: Box<T>,
}

impl<T, Ty> LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Typ, B>(v: &str, sup: Typ, bod: B) -> LambdaSub<T, Ty>
    where
        Typ: Into<Ty>,
        B: Into<T>,
    {
        LambdaSub {
            var: v.to_owned(),
            sup_ty: sup.into(),
            body: Box::new(bod.into()),
        }
    }

    pub fn new_unbounded<T1>(v: &str, bod: T1) -> LambdaSub<T, Ty>
    where
        T1: Into<T>,
        Top<Ty>: Into<Ty>,
    {
        LambdaSub {
            var: v.to_owned(),
            sup_ty: Top::new_star().into(),
            body: Box::new(bod.into()),
        }
    }
}

impl<T, Ty> Term for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for LambdaSub<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
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

impl<T, Ty> SubstType<Ty> for LambdaSub<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
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

impl<T, Ty> fmt::Display for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:({}).{}", self.var, self.sup_ty, self.body)
    }
}
