use super::Term;
use crate::{
    check::{Kindcheck, Typecheck},
    errors::Error,
    eval::{Eval, Normalize},
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    values::Exception as ExceptionVal,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exception<T>
where
    T: LanguageTerm,
{
    ty: <T as LanguageTerm>::Type,
    phantom: PhantomData<T>,
}

impl<T> Exception<T>
where
    T: LanguageTerm,
{
    pub fn new<Typ>(ty: Typ) -> Exception<T>
    where
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        Exception {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Exception<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Exception<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Exception<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Exception {
            ty: self.ty.subst_type(v, ty),
            phantom: PhantomData,
        }
        .into()
    }
}

impl<T> Typecheck for Exception<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        ty_norm.check_kind(env)?;
        Ok(ty_norm)
    }
}

impl<T> Eval for Exception<T>
where
    T: LanguageTerm,
    ExceptionVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(ExceptionVal::new(self.ty).into())
    }
}

impl<T> fmt::Display for Exception<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
