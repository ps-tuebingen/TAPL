use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpApp<Ty>
where
    Ty: Type,
{
    fun: Box<Ty>,
    arg: Box<Ty>,
}

impl<Ty> OpApp<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(fun: Ty1, arg: Ty2) -> OpApp<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        OpApp {
            fun: Box::new(fun.into()),
            arg: Box::new(arg.into()),
        }
    }
}

impl<Ty> Type for OpApp<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for OpApp<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        OpApp {
            fun: Box::new(self.fun.subst_type(v, ty)),
            arg: Box::new(self.arg.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for OpApp<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})[{}]", self.fun, self.arg)
    }
}
