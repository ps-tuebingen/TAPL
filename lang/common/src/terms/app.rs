use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval, EvalEnvironment},
    subst::{SubstTerm, SubstType},
    types::Type,
    values::Value,
    TypeVar, Var,
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

impl<Env, Ty, T> Typecheck<Env, Ty> for App<T>
where
    Env: CheckEnvironment<Ty>,
    Ty: Type,
    T: Term + Typecheck<Env, Ty>,
{
    fn check(&self, env: &mut Env) -> Result<Ty, Error> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let fun = fun_ty.into_fun().map_err(to_check_err)?;
        let arg_ty = self.arg.check(env)?;
        if *fun.from == arg_ty {
            Ok(*fun.to)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: arg_ty.to_string(),
                expected: fun.from.to_string(),
            }))
        }
    }
}

impl<V, Env, T, Ty> Eval<V, Env, T, Ty> for App<T>
where
    V: Value<T>,
    T: Term + SubstTerm<T, Target = T> + Eval<V, Env, T, Ty>,
    Env: EvalEnvironment,
    Ty: Type,
{
    fn eval(self, env: &mut Env) -> Result<V, Error> {
        let fun_val = self.fun.eval(env)?;
        let arg_val: V::Term = self.arg.eval(env)?.into();
        let lam = fun_val.into_lambda::<Ty>().map_err(to_eval_err)?;
        lam.body.subst(&lam.var, &arg_val.into()).eval(env)
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
