use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::{to_eval_err, Eval, Normalize},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
}

impl<T> Fix<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(t: T1) -> Fix<T>
    where
        T1: Into<T>,
    {
        Fix {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Fix<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Fix<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Fix {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Fix<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Fix {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Fix<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let fun_ty = term_ty.into_fun().map_err(to_check_err)?;
        fun_ty.from.check_equal(&fun_ty.to).map_err(to_check_err)?;
        Ok(*fun_ty.from)
    }
}

impl<T> Eval for Fix<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.clone().eval(env)?;
        let lam_val = term_val.into_lambda().map_err(to_eval_err)?;
        lam_val.body.subst(&lam_val.var, &self.into()).eval(env)
    }
}

impl<T> fmt::Display for Fix<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix({})", self.term)
    }
}
