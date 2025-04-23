use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::{Eval, Normalize},
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Optional,
    values::Something as SomethingVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Something<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
}

impl<T> Something<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(t: T1) -> Something<T>
    where
        T1: Into<T>,
    {
        Something {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Something<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Something<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Something {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Something<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Something {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Something<T>
where
    T: LanguageTerm,
    Optional<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.term.check(env)?.normalize(env);
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        Ok(Optional::new(term_ty.clone()).into())
    }
}

impl<T> Eval for Something<T>
where
    T: LanguageTerm,
    SomethingVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(SomethingVal::<T>::new(term_val).into())
    }
}

impl<T> fmt::Display for Something<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.term)
    }
}
