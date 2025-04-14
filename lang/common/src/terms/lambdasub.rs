use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    var: Var,
    sup_ty: Ty,
    body: Box<T>,
}

impl<T, Ty> SubstTerm<T> for LambdaSub<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
    Ty: Type,
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
        write!(f, "\\{}<:{}.{}", self.var, self.sup_ty, self.body)
    }
}
