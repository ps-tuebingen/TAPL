use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Kindcheck, Typecheck},
    errors::Error,
    eval::Eval,
    kinds::Kind,
    language::{LanguageTerm, LanguageType},
    subst::{SubstTerm, SubstType},
    types::Mu,
    values::Fold as FoldVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fold<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Fold<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Typ>(t: T1, ty: Typ) -> Fold<T>
    where
        T1: Into<T>,
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        Fold {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Fold<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Fold<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Fold {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Fold<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Fold {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for Fold<T>
where
    T: LanguageTerm,
    Mu<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let mu_ty = self.ty.clone().into_mu().map_err(to_check_err)?;
        env.add_tyvar_kind(mu_ty.var.clone(), Kind::Star);
        mu_ty
            .ty
            .check_kind(env)?
            .into_star()
            .map_err(to_check_err)?;

        let mu_subst = mu_ty
            .ty
            .clone()
            .subst_type(&mu_ty.var.clone(), &(mu_ty.into()));
        let term_ty = self.term.check(env)?;
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        term_ty.check_equal(&mu_subst).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}

impl<T> Eval for Fold<T>
where
    T: LanguageTerm,
    FoldVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(FoldVal::<T>::new(self.ty, term_val).into())
    }
}

impl<T> fmt::Display for Fold<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.term)
    }
}
