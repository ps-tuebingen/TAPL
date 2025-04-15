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
pub struct If<T>
where
    T: Term,
{
    if_cond: Box<T>,
    then_term: Box<T>,
    else_term: Box<T>,
}

impl<T> If<T>
where
    T: Term,
{
    pub fn new<T1, T2, T3>(cond: T1, th: T2, els: T3) -> If<T>
    where
        T1: Into<T>,
        T2: Into<T>,
        T3: Into<T>,
    {
        If {
            if_cond: Box::new(cond.into()),
            then_term: Box::new(th.into()),
            else_term: Box::new(els.into()),
        }
    }
}

impl<T> Term for If<T> where T: Term {}

impl<T> SubstTerm<T> for If<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        If {
            if_cond: Box::new(self.if_cond.subst(v, t)),
            then_term: Box::new(self.then_term.subst(v, t)),
            else_term: Box::new(self.else_term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for If<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        If {
            if_cond: Box::new(self.if_cond.subst_type(v, ty)),
            then_term: Box::new(self.then_term.subst_type(v, ty)),
            else_term: Box::new(self.else_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Env, Ty, T> Typecheck<Env, Ty> for If<T>
where
    T: Term + Typecheck<Env, Ty>,
    Ty: Type,
    Env: CheckEnvironment<Ty>,
{
    fn check(&self, env: &mut Env) -> Result<Ty, Error> {
        let if_ty = self.if_cond.check(&mut env.clone())?;
        if_ty.into_bool().map_err(to_check_err)?;
        let then_ty = self.then_term.check(&mut env.clone())?;
        let else_ty = self.else_term.check(env)?;
        if then_ty != else_ty {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: then_ty.to_string(),
                expected: else_ty.to_string(),
            }))
        } else {
            Ok(then_ty)
        }
    }
}

impl<Val, Env, T, Ty> Eval<Val, Env, T, Ty> for If<T>
where
    T: Term + SubstTerm<T, Target = T> + Eval<Val, Env, T, Ty>,
    Ty: Type,
    Val: Value<T>,
    Env: EvalEnvironment,
{
    fn eval(self, env: &mut Env) -> Result<Val, Error> {
        let cond_val = self.if_cond.eval(env)?;
        let cond_val_str = cond_val.to_string();
        if cond_val.clone().into_true().is_ok() {
            self.then_term.eval(env)
        } else if cond_val.into_false().is_ok() {
            self.else_term.eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: cond_val_str,
                expected: "Boolean".to_owned(),
            }))
        }
    }
}

impl<T> fmt::Display for If<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.if_cond, self.then_term, self.else_term
        )
    }
}
