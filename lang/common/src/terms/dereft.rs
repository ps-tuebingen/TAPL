use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::{to_eval_err, Eval, EvalEnvironment, Normalize},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deref<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
}

impl<T> Deref<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(t: T1) -> Deref<T>
    where
        T1: Into<T>,
    {
        Deref {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Deref<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Deref<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Deref {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Deref<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Deref {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Deref<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.term.check(env)?.normalize(env);
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let ref_ty = term_ty.into_ref().map_err(to_check_err)?;
        Ok(*ref_ty.ty)
    }
}

impl<T> Eval for Deref<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.clone().eval(env)?;
        let loc_val = term_val.into_loc().map_err(to_eval_err)?;
        env.get_location(loc_val.loc).map_err(to_eval_err)
    }
}

impl<T> fmt::Display for Deref<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "!{}", self.term)
    }
}
