use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nil<T, Ty>
where
    Ty: Type,
    T: Term,
{
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<T, Ty> Term for Nil<T, Ty>
where
    Ty: Type,
    T: Term,
{
}

impl<T, Ty> SubstTerm<T> for Nil<T, Ty>
where
    T: Term,
    Self: Into<T>,
    Ty: Type,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for Nil<T, Ty>
where
    T: Term,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Nil {
            ty: self.ty.subst_type(v, ty),
            phantom: PhantomData,
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Nil<T, Ty>
where
    Ty: Type,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nil[{}]", self.ty)
    }
}
