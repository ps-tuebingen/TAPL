use super::Term;
use crate::{
    check::Typecheck,
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Optional,
    values::Nothing as NothingVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nothing<T>
where
    T: LanguageTerm,
{
    ty: <T as LanguageTerm>::Type,
}

impl<T> Nothing<T>
where
    T: LanguageTerm,
{
    pub fn new<Typ>(ty: Typ) -> Nothing<T>
    where
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        Nothing { ty: ty.into() }
    }
}

impl<T> Term for Nothing<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Nothing<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Nothing<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Nothing {
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for Nothing<T>
where
    T: LanguageTerm,
    Optional<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Optional::new(self.ty.clone()).into())
    }
}

impl<T> Eval for Nothing<T>
where
    T: LanguageTerm,
    NothingVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NothingVal::<T>::new(self.ty).into())
    }
}

impl<T> fmt::Display for Nothing<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nothing[{}]", self.ty)
    }
}
