use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval, EvalEnvironment},
    subst::{SubstTerm, SubstType},
    terms::{App, Raise},
    types::Type,
    values::Value,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TryWithVal<T>
where
    T: Term,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> TryWithVal<T>
where
    T: Term,
{
    pub fn new<T1, T2>(t: T1, h: T2) -> TryWithVal<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        TryWithVal {
            term: Box::new(t.into()),
            handler: Box::new(h.into()),
        }
    }
}

impl<T> Term for TryWithVal<T> where T: Term {}

impl<T> SubstTerm<T> for TryWithVal<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TryWithVal {
            term: Box::new(self.term.subst(v, t)),
            handler: Box::new(self.handler.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for TryWithVal<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        TryWithVal {
            term: Box::new(self.term.subst_type(v, ty)),
            handler: Box::new(self.handler.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Env, Ty, T> Typecheck<Env, Ty> for TryWithVal<T>
where
    T: Term + Typecheck<Env, Ty>,
    Ty: Type,
    Env: CheckEnvironment<Ty>,
{
    fn check(&self, env: &mut Env) -> Result<Ty, Error> {
        let t_ty = self.term.check(&mut env.clone())?;
        let handler_ty = self.handler.check(env)?;
        let fun = handler_ty.into_fun().map_err(to_check_err)?;
        if t_ty == *fun.to {
            Ok(t_ty)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: t_ty.to_string(),
                expected: fun.to.to_string(),
            }))
        }
    }
}

impl<Val, Env, T, Ty> Eval<Val, Env, T, Ty> for TryWithVal<T>
where
    T: Term + Eval<Val, Env, T, Ty> + SubstTerm<T, Target = T>,
    Ty: Type,
    Val: Value<T> + Into<T>,
    Env: EvalEnvironment,
    Raise<T, Ty>: Into<T>,
{
    fn eval(self, env: &mut Env) -> Result<Val, Error> {
        let term_evaled = self.term.eval(env)?;
        let raise: Raise<T, Ty> = term_evaled
            .into_raise::<Val, Ty>()
            .map_err(to_eval_err)?
            .into();

        App::new(*self.handler, raise).eval(env)
    }
}

impl<T> fmt::Display for TryWithVal<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} catch {{ {} }}", self.term, self.handler)
    }
}
