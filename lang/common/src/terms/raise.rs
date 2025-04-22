use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::Eval,
    language::{LanguageTerm, LanguageType},
    subst::{SubstTerm, SubstType},
    values::Raise as RaiseVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raise<T>
where
    T: LanguageTerm,
{
    exception: Box<T>,
    exception_ty: <T as LanguageTerm>::Type,
    cont_ty: <T as LanguageTerm>::Type,
}

impl<T> Raise<T>
where
    T: LanguageTerm,
{
    pub fn new<E, Ty1, Ty2>(ex: E, ex_ty: Ty1, cont_ty: Ty2) -> Raise<T>
    where
        E: Into<T>,
        Ty1: Into<<T as LanguageTerm>::Type>,
        Ty2: Into<<T as LanguageTerm>::Type>,
    {
        Raise {
            exception: Box::new(ex.into()),
            exception_ty: ex_ty.into(),
            cont_ty: cont_ty.into(),
        }
    }
}

impl<T> Term for Raise<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Raise<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Raise {
            exception: Box::new(self.exception.subst(v, t)),
            exception_ty: self.exception_ty,
            cont_ty: self.cont_ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Raise<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Raise {
            exception: Box::new(self.exception.subst_type(v, ty)),
            exception_ty: self.exception_ty.subst_type(v, ty),
            cont_ty: self.cont_ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for Raise<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ex_knd = self.exception_ty.check_kind(env)?;
        self.cont_ty.check_kind(env)?;

        let err_ty = self.exception.check(env)?;
        let err_knd = err_ty.check_kind(env)?;

        ex_knd.check_equal(&err_knd).map_err(to_check_err)?;
        self.exception_ty
            .check_equal(&err_ty)
            .map_err(to_check_err)?;

        Ok(self.cont_ty.clone())
    }
}

impl<T> Eval for Raise<T>
where
    T: LanguageTerm,
    RaiseVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let exc_val = self.exception.eval(env)?;
        let raise_val = RaiseVal::<T>::new(exc_val, self.cont_ty, self.exception_ty);
        Ok(raise_val.into())
    }
}

impl<T> fmt::Display for Raise<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{}]({} : {})",
            self.cont_ty, self.exception, self.exception_ty
        )
    }
}
