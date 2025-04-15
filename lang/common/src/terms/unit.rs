use super::Term;
use crate::{
    check::{CheckEnvironment, Typecheck},
    errors::Error,
    eval::{Eval, EvalEnvironment},
    subst::{SubstTerm, SubstType},
    types::{Type, Unit as UnitTy},
    values::{Unit as UnitVal, Value},
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> Unit<T>
where
    T: Term,
{
    pub fn new() -> Unit<T> {
        Unit {
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Unit<T> where T: Term {}

impl<T> SubstTerm<T> for Unit<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for Unit<T>
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

impl<Env, Ty, T> Typecheck<Env, Ty> for Unit<T>
where
    T: Term,
    Ty: Type,
    UnitTy: Into<Ty>,
    Env: CheckEnvironment<Ty>,
{
    fn check(&self, _: &mut Env) -> Result<Ty, Error> {
        Ok(UnitTy.into())
    }
}

impl<Val, Env, T, Ty> Eval<Val, Env, T, Ty> for Unit<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
    Val: Value<T>,
    Env: EvalEnvironment,
    UnitVal: Into<Val>,
{
    fn eval(self, _: &mut Env) -> Result<Val, Error> {
        Ok(UnitVal.into())
    }
}

impl<T> fmt::Display for Unit<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}
