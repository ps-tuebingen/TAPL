use super::Term;
use crate::{
    check::{CheckEnvironment, Typecheck},
    errors::Error,
    eval::{Eval, EvalEnvironment},
    subst::{SubstTerm, SubstType},
    types::{Nat, Type},
    values::{Num as NumVal, Value},
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Num<T>
where
    T: Term,
{
    num: i64,
    phantom: PhantomData<T>,
}

impl<T> Num<T>
where
    T: Term,
{
    pub fn new(num: i64) -> Num<T> {
        Num {
            num,
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Num<T> where T: Term {}

impl<T> SubstTerm<T> for Num<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for Num<T>
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

impl<Env, Ty, T> Typecheck<Env, Ty> for Num<T>
where
    T: Term,
    Env: CheckEnvironment<Ty>,
    Ty: Type,
    Nat: Into<Ty>,
{
    fn check(&self, _: &mut Env) -> Result<Ty, Error> {
        Ok(Nat.into())
    }
}

impl<Val, Env, Ty, T> Eval<Val, Env, T, Ty> for Num<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
    Val: Value<T>,
    Env: EvalEnvironment,
    NumVal: Into<Val>,
{
    fn eval(self, _: &mut Env) -> Result<Val, Error> {
        Ok(NumVal::new(self.num).into())
    }
}

impl<T> fmt::Display for Num<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
