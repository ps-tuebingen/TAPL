use super::Term;
use common::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::{Eval, Normalize},
    language::{LanguageTerm, LanguageType},
    subst::{SubstTerm, SubstType},
    values::Right as RightVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Right<T>
where
    T: LanguageTerm,
{
    right_term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Right<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty1>(right_t: T1, ty: Ty1) -> Right<T>
    where
        T1: Into<T>,
        Ty1: Into<<T as LanguageTerm>::Type>,
    {
        Right {
            right_term: Box::new(right_t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Right<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Right<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Right {
            right_term: Box::new(self.right_term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Right<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Right {
            right_term: Box::new(self.right_term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for Right<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let right_ty = self
            .right_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let right_knd = right_ty.check_kind(&mut env.clone())?;

        let sum_ty = self
            .ty
            .clone()
            .normalize(&mut env.clone())
            .into_sum()
            .map_err(to_check_err)?;
        let sum_knd = sum_ty.check_kind(env)?;

        right_knd.check_equal(&sum_knd).map_err(to_check_err)?;
        sum_ty.right.check_equal(&right_ty).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}

impl<T> Eval for Right<T>
where
    T: LanguageTerm,
    RightVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let right_val = self.right_term.eval(env)?;
        Ok(RightVal::<T>::new(right_val, self.ty.clone()).into())
    }
}

impl<T> fmt::Display for Right<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.right_term, self.ty)
    }
}
