use super::Term;
use crate::{
    check::{CheckEnvironment, Typecheck},
    errors::Error,
    eval::{Eval, EvalEnvironment},
    subst::{SubstTerm, SubstType},
    types::{Bool, Type},
    values::{True as TrueVal, Value},
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct True<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> True<T>
where
    T: Term,
{
    pub fn new() -> True<T> {
        True {
            phantom: PhantomData,
        }
    }
}

impl<T> Term for True<T> where T: Term {}

impl<T> SubstTerm<T> for True<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for True<T>
where
    T: Term,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Env, Ty, T> Typecheck<Env, Ty> for True<T>
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

impl<Val, Env, T, Ty> Eval<Val, Env, T, Ty> for True<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
    Val: Value<T>,
    Env: EvalEnvironment,
    TrueVal: Into<Val>,
{
    fn eval(self, _: &mut Env) -> Result<Val, Error> {
        Ok(TrueVal.into())
    }
}

impl<T> fmt::Display for True<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
