use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raise<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub exception: Box<T>,
    pub exception_ty: Ty,
    pub cont_ty: Ty,
}

impl<T, Ty> Raise<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<E, Ty1, Ty2>(ex: E, ex_ty: Ty1, cont_ty: Ty2) -> Raise<T, Ty>
    where
        E: Into<T>,
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        Raise {
            exception: Box::new(ex.into()),
            exception_ty: ex_ty.into(),
            cont_ty: cont_ty.into(),
        }
    }
}

impl<T, Ty> Term for Raise<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Raise<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Raise {
            exception: Box::new(self.exception.subst(v, t)),
            exception_ty: self.exception_ty,
            cont_ty: self.cont_ty,
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Raise<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Raise {
            exception: Box::new(self.exception.subst_type(v, ty)),
            exception_ty: self.exception_ty.subst_type(v, ty),
            cont_ty: self.cont_ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Raise<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{},{}]({})",
            self.cont_ty, self.exception_ty, self.exception,
        )
    }
}
