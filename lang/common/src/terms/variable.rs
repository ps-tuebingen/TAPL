use super::Term;
use crate::{
    check::{CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval, EvalEnvironment},
    subst::{SubstTerm, SubstType},
    types::Type,
    values::Value,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable<T>
where
    T: Term,
{
    var: Var,
    phantom: PhantomData<T>,
}

impl<T: Term> Variable<T> {
    pub fn new(v: &str) -> Variable<T> {
        Variable {
            var: v.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Variable<T> where T: Term {}

impl<T> SubstTerm<T> for Variable<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            t.clone()
        }
    }
}

impl<T, Ty> SubstType<Ty> for Variable<T>
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

impl<T, Ty, Env> Typecheck<Env, Ty> for Variable<T>
where
    T: Term,
    Ty: Type,
    Env: CheckEnvironment<Ty>,
{
    fn check(&self, env: &mut Env) -> Result<Ty, Error> {
        env.get_var(&self.var)
    }
}

impl<Val, Env, T, Ty> Eval<Val, Env, T, Ty> for Variable<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
    Env: EvalEnvironment,
    Val: Value<T>,
{
    fn eval(self, _: &mut Env) -> Result<Val, Error> {
        Err(to_eval_err(ErrorKind::FreeVariable(self.var)))
    }
}

impl<T> fmt::Display for Variable<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}
