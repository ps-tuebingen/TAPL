use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::List,
    values::Nil as NilVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nil<T>
where
    T: LanguageTerm,
{
    ty: <T as LanguageTerm>::Type,
}

impl<T> Nil<T>
where
    T: LanguageTerm,
{
    pub fn new<Typ>(ty: Typ) -> Nil<T>
    where
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        Nil { ty: ty.into() }
    }
}

impl<T> Term for Nil<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Nil<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Nil<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Nil {
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for Nil<T>
where
    T: LanguageTerm,
    List<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        self.ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        Ok(List::new(self.ty.clone()).into())
    }
}

impl<T> Eval for Nil<T>
where
    T: LanguageTerm,
    NilVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NilVal::<T>::new(self.ty).into())
    }
}

impl<T> fmt::Display for Nil<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nil[{}]", self.ty)
    }
}
