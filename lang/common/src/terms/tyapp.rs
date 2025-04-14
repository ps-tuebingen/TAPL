use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct TyApp<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fun: Box<T>,
    arg: Ty,
}

impl<T, Ty> Term for TyApp<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for TyApp<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
    Ty: Type,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TyApp {
            fun: Box::new(self.fun.subst(v, t)),
            arg: self.arg,
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for TyApp<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        TyApp {
            fun: Box::new(self.fun.subst_type(v, ty)),
            arg: self.arg.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for TyApp<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})[{}]", self.fun, self.arg)
    }
}
