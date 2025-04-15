use super::Term;
use crate::{
    check::{CheckEnvironment, Typecheck},
    errors::Error,
    eval::{Eval, EvalEnvironment},
    subst::{SubstTerm, SubstType},
    types::{Bool, Type},
    values::{False as FalseVal, Value},
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct False<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> False<T>
where
    T: Term,
{
    pub fn new() -> False<T> {
        False {
            phantom: PhantomData,
        }
    }
}

impl<T> Term for False<T> where T: Term {}

impl<T> SubstTerm<T> for False<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for False<T>
where
    T: Term,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        False {
            phantom: PhantomData,
        }
        .into()
    }
}

impl<T, Ty, Env> Typecheck<Env, Ty> for False<T>
where
    T: Term,
    Ty: Type,
    Env: CheckEnvironment<Ty>,
    Bool: Into<Ty>,
{
    fn check(&self, _: &mut Env) -> Result<Ty, Error> {
        Ok(Bool.into())
    }
}

impl<Val, Env, T, Ty> Eval<Val, Env, T, Ty> for False<T>
where
    Val: Value<T>,
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
    Env: EvalEnvironment,
    FalseVal: Into<Val>,
{
    fn eval(self, _: &mut Env) -> Result<Val, Error> {
        Ok(FalseVal.into())
    }
}

impl<T> fmt::Display for False<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}
