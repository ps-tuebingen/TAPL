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
pub struct Snd<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
}

impl<T> Snd<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(t: T1) -> Snd<T>
    where
        T1: Into<T>,
    {
        Snd {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Snd<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Snd<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Snd {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Snd<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Snd {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Snd<T>
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
        term_ty.check_kind(env)?;
        let prod_ty = term_ty.into_product().map_err(to_check_err)?;
        Ok(*prod_ty.snd)
    }
}

impl<T> Eval for Snd<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let pair_val = term_val.into_pair().map_err(to_eval_err)?;
        Ok(*pair_val.snd)
    }
}

impl<T> fmt::Display for Snd<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).1", self.term)
    }
}
