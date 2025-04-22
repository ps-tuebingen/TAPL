use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::{to_eval_err, Eval},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    terms::{App, Raise},
    types::Fun,
    values::Value,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TryWithVal<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> TryWithVal<T>
where
    T: LanguageTerm,
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

impl<T> Term for TryWithVal<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for TryWithVal<T>
where
    T: LanguageTerm,
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

impl<T> SubstType<<T as LanguageTerm>::Type> for TryWithVal<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        TryWithVal {
            term: Box::new(self.term.subst_type(v, ty)),
            handler: Box::new(self.handler.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for TryWithVal<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let t_ty = self.term.check(&mut env.clone())?;
        let t_knd = t_ty.check_kind(env)?;

        let handler_ty = self.handler.check(env)?;
        let handler_knd = handler_ty.check_kind(env)?;
        let fun: Fun<<T as LanguageTerm>::Type> = handler_ty.into_fun().map_err(to_check_err)?;

        t_knd.check_equal(&handler_knd).map_err(to_check_err)?;
        fun.to.check_equal(&t_ty).map_err(to_check_err)?;
        Ok(t_ty)
    }
}

impl<T> Eval for TryWithVal<T>
where
    T: LanguageTerm,
    Raise<T>: Into<T>,
    App<T>: Into<T>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_evaled = self.term.eval(env)?;
        let raise = term_evaled.into_raise().map_err(to_eval_err)?.into_term();
        App::new(*self.handler, raise).into().eval(env)
    }
}

impl<T> fmt::Display for TryWithVal<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} catch {{ {} }}", self.term, self.handler)
    }
}
