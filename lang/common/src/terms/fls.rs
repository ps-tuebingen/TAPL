use super::Term;
use crate::{
    check::Typecheck,
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Bool,
    values::False as FalseVal,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct False<T>
where
    T: LanguageTerm,
{
    phantom: PhantomData<T>,
}

impl<T> False<T>
where
    T: LanguageTerm,
{
    pub fn new() -> False<T> {
        False {
            phantom: PhantomData,
        }
    }
}

impl<T> Default for False<T>
where
    T: LanguageTerm,
{
    fn default() -> False<T> {
        False::new()
    }
}

impl<T> Term for False<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for False<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for False<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &<T as LanguageTerm>::Type) -> Self::Target {
        False {
            phantom: PhantomData,
        }
        .into()
    }
}

impl<T> Typecheck for False<T>
where
    T: LanguageTerm,
    Bool<<T as LanguageTerm>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Bool::new().into())
    }
}

impl<T> Eval for False<T>
where
    T: LanguageTerm,
    FalseVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(FalseVal::new().into())
    }
}

impl<T> fmt::Display for False<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}
