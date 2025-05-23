use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raise<T>
where
    T: Term,
{
    exception: Box<T>,
    exception_ty: <T as Term>::Type,
    cont_ty: <T as Term>::Type,
}

impl<T> Raise<T>
where
    T: Term,
{
    pub fn new<E, Ty1, Ty2>(ex: E, ex_ty: Ty1, cont_ty: Ty2) -> Raise<T>
    where
        E: Into<T>,
        Ty1: Into<<T as Term>::Type>,
        Ty2: Into<<T as Term>::Type>,
    {
        Raise {
            exception: Box::new(ex.into()),
            exception_ty: ex_ty.into(),
            cont_ty: cont_ty.into(),
        }
    }
}

impl<T> Term for Raise<T> where T: Term {}

impl<T> SubstTerm<T> for Raise<T>
where
    T: Term,
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

impl<T> SubstType<<T as Term>::Type> for Raise<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Raise {
            exception: Box::new(self.exception.subst_type(v, ty)),
            exception_ty: self.exception_ty.subst_type(v, ty),
            cont_ty: self.cont_ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Raise<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{}]({} : {})",
            self.cont_ty, self.exception, self.exception_ty
        )
    }
}
