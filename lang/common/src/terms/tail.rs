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
pub struct Tail<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Tail<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty>(t: T1, ty: Ty) -> Tail<T>
    where
        T1: Into<T>,
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        Tail {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Tail<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Tail<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Tail {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Tail<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Tail {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for Tail<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.term.check(env)?.normalize(env);
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let list_ty = term_ty.into_list().map_err(to_check_err)?;
        Ok(*list_ty.ty)
    }
}

impl<T> Eval for Tail<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let cons_val = term_val.into_cons().map_err(to_eval_err)?;
        Ok(*cons_val.head)
    }
}

impl<T> fmt::Display for Tail<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tail[{}]({})", self.term, self.term)
    }
}
