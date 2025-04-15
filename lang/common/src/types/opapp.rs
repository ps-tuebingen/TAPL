use super::{Fun, Type};
use crate::{errors::ErrorKind, subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpApp<Ty>
where
    Ty: Type,
{
    fun: Box<Ty>,
    arg: Box<Ty>,
}

impl<Ty> Type for OpApp<Ty>
where
    Ty: Type,
{
    fn into_fun<Ty1>(self) -> Result<Fun<Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Function Type".to_owned(),
        })
    }
}

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
