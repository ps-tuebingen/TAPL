use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    eval::{Eval, EvalEnvironment},
    subst::{SubstTerm, SubstType},
    types::Type,
    values::Value,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Try<T>
where
    T: Term,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> Try<T>
where
    T: Term,
{
    pub fn new<T1: Into<T>, T2: Into<T>>(t: T1, h: T2) -> Try<T> {
        Try {
            term: Box::new(t.into()),
            handler: Box::new(h.into()),
        }
    }
}

impl<T> Term for Try<T> where T: Term {}

impl<T> SubstTerm<T> for Try<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Try {
            term: Box::new(self.term.subst(v, t)),
            handler: Box::new(self.handler.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Try<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Try {
            term: Box::new(self.term.subst_type(v, ty)),
            handler: Box::new(self.handler.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Env, Ty, T> Typecheck<Env, Ty> for Try<T>
where
    Env: CheckEnvironment<Ty>,
    Ty: Type,
    T: Term + Typecheck<Env, Ty>,
{
    fn check(&self, env: &mut Env) -> Result<Ty, Error> {
        let term_ty = self.term.check(&mut env.clone())?;
        let handler_ty = self.handler.check(env)?;
        if term_ty == handler_ty {
            Ok(term_ty)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: term_ty.to_string(),
                expected: handler_ty.to_string(),
            }))
        }
    }
}

impl<Val, Env, T, Ty> Eval<Val, Env, T, Ty> for Try<T>
where
    T: Term + Eval<Val, Env, T, Ty> + SubstTerm<T, Target = T>,
    Ty: Type,
    Val: Value<T>,
    Env: EvalEnvironment,
{
    fn eval(self, env: &mut Env) -> Result<Val, Error> {
        let term_val = self.term.eval(env);
        if term_val.is_err() {
            self.handler.eval(env)
        } else {
            term_val
        }
    }
}

impl<T> fmt::Display for Try<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} with {{ {} }}", self.term, self.handler)
    }
}
