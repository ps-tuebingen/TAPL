use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval, Normalize},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If<T>
where
    T: LanguageTerm,
{
    if_cond: Box<T>,
    then_term: Box<T>,
    else_term: Box<T>,
}

impl<T> If<T>
where
    T: LanguageTerm,
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

impl<T> Term for If<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for If<T>
where
    T: LanguageTerm,
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

impl<T> SubstType<<T as LanguageTerm>::Type> for If<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        If {
            if_cond: Box::new(self.if_cond.subst_type(v, ty)),
            then_term: Box::new(self.then_term.subst_type(v, ty)),
            else_term: Box::new(self.else_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for If<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let if_ty = self
            .if_cond
            .check(&mut &mut env.clone().clone())?
            .normalize(&mut env.clone());
        if_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;
        if_ty.into_bool().map_err(to_check_err)?;

        let then_ty = self
            .then_term
            .check(&mut &mut env.clone().clone())?
            .normalize(&mut env.clone());
        let then_kind = then_ty.check_kind(&mut env.clone())?;

        let else_ty = self
            .else_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let else_kind = else_ty.check_kind(env)?;

        then_kind.check_equal(&else_kind).map_err(to_check_err)?;
        then_ty.check_equal(&else_ty).map_err(to_check_err)?;
        Ok(then_ty)
    }
}

impl<T> Eval for If<T>
where
    T: LanguageTerm,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
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
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.if_cond, self.then_term, self.else_term
        )
    }
}
