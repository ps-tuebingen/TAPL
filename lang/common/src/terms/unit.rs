use super::Term;
use crate::{
    check::Typecheck,
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::{Type, Unit as UnitTy},
    values::Unit as UnitVal,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit<T>
where
    T: LanguageTerm,
{
    phantom: PhantomData<T>,
}

impl<T> Unit<T>
where
    T: LanguageTerm,
{
    pub fn new() -> Unit<T> {
        Unit {
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Unit<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Unit<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for Unit<T>
where
    T: LanguageTerm,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<T> Typecheck for Unit<T>
where
    T: LanguageTerm,
    UnitTy<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(UnitTy::new().into())
    }
}

impl<T> Eval for Unit<T>
where
    T: LanguageTerm,
    UnitVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(UnitVal::new().into())
    }
}

impl<T> fmt::Display for Unit<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}
