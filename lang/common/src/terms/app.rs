use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct App<T>
where
    T: Term,
{
    pub fun: Box<T>,
    pub arg: Box<T>,
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
