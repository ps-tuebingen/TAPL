use super::Term;
use crate::{
    check::{to_check_err, Typecheck},
    errors::Error,
    eval::Eval,
    language::{LanguageTerm, LanguageType},
    subst::{SubstTerm, SubstType},
    values::Left as LeftVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Left<T>
where
    T: LanguageTerm,
{
    left_term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Left<T>
where
    T: LanguageTerm,
{
    pub fn new<L, Typ>(left_t: L, ty: Typ) -> Left<T>
    where
        L: Into<T>,
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        Left {
            left_term: Box::new(left_t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Left<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Left<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Left {
            left_term: Box::new(self.left_term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Left<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Left {
            left_term: Box::new(self.left_term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for Left<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let left_ty = self.left_term.check(env)?;
        let sum_ty = self.ty.clone().into_sum().map_err(to_check_err)?;
        sum_ty.left.check_equal(&left_ty).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}

impl<T> Eval for Left<T>
where
    T: LanguageTerm,
    LeftVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let left_val = self.left_term.eval(env)?;
        Ok(LeftVal::<T>::new(left_val, self.ty).into())
    }
}

impl<T> fmt::Display for Left<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_term, self.ty)
    }
}
