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
pub struct Unfold<T>
where
    T: LanguageTerm,
{
    ty: <T as LanguageTerm>::Type,
    term: Box<T>,
}

impl<T> Unfold<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty>(ty: Ty, t: T1) -> Unfold<T>
    where
        T1: Into<T>,
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        Unfold {
            ty: ty.into(),
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Unfold<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Unfold<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Unfold {
            ty: self.ty,
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Unfold<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Unfold {
            ty: self.ty.subst_type(v, ty),
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Unfold<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let ty_kind = ty_norm.check_kind(&mut env.clone())?;

        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let term_knd = term_ty.check_kind(env)?;
        term_knd.check_equal(&ty_kind).map_err(to_check_err)?;

        ty_norm.check_equal(&term_ty).map_err(to_check_err)?;
        let mu_ty = term_ty.clone().into_mu().map_err(to_check_err)?;
        Ok(mu_ty.ty.subst_type(&mu_ty.var, &term_ty))
    }
}

impl<T> Eval for Unfold<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let term_fold = term_val.into_fold().map_err(to_eval_err)?;
        Ok(*term_fold.val)
    }
}

impl<T> fmt::Display for Unfold<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unfold[{}]({})", self.ty, self.term)
    }
}
