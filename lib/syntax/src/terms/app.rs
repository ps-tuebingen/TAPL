use super::{Lambda, Term};
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::{Type, Unit as UnitTy},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct App<T>
where
    T: Term,
{
    pub fun: Box<T>,
    pub arg: Box<T>,
}

impl<T> App<T>
where
    T: Term,
{
    pub fn new<F: Into<T>, A: Into<T>>(f: F, a: A) -> App<T> {
        App {
            fun: Box::new(f.into()),
            arg: Box::new(a.into()),
        }
    }

    pub fn seq<T1, T2, Ty>(t1: T1, t2: T2) -> App<T>
    where
        Ty: Type,
        T1: Into<T>,
        T2: Into<T>,
        Lambda<T, Ty>: Into<T>,
        UnitTy<Ty>: Into<Ty>,
    {
        App {
            fun: Box::new(Lambda::new("_", UnitTy::new(), t2).into()),
            arg: Box::new(t1.into()),
        }
    }
}

impl<T> Term for App<T> where T: Term {}

impl<T> SubstTerm<T> for App<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> Self::Target {
        App {
            fun: Box::new(self.fun.subst(v, t)),
            arg: Box::new(self.arg.subst(v, t)),
        }
        .into()
    }
}
impl<Ty, T> SubstType<Ty> for App<T>
where
    Ty: Type,
    T: Term + SubstType<Ty, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        App {
            fun: Box::new(self.fun.subst_type(v, ty)),
            arg: Box::new(self.arg.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for App<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
