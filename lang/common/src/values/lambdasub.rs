use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{LambdaSub as LambdaSubT, Term},
    types::Type,
    Var,
};
use std::fmt;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    var: Var,
    sup_ty: Ty,
    t: T,
}

impl<T, Ty> Value<T> for LambdaSub<T, Ty>
where
    T: Term + From<LambdaSubT<T, Ty>>,
    Ty: Type,
{
    type Term = LambdaSubT<T, Ty>;
    fn into_lambda<Ty1>(self) -> Result<Lambda<T, Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_raise<Val, Ty1>(self) -> Result<Raise<Val, Ty1, T>, ErrorKind>
    where
        Val: Value<T>,
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Raise".to_owned(),
        })
    }
}

impl<T, Ty> From<LambdaSub<T, Ty>> for LambdaSubT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(lam: LambdaSub<T, Ty>) -> LambdaSubT<T, Ty> {
        LambdaSubT::new(&lam.var, lam.sup_ty, lam.t)
    }
}

impl<T, Ty> fmt::Display for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup_ty, self.t)
    }
}
