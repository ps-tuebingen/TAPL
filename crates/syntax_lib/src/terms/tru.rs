use super::Term;
use common::{
    check::Typecheck,
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::{Bool, Type},
    values::True as TrueVal,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct True<T>
where
    T: LanguageTerm,
{
    phantom: PhantomData<T>,
}

impl<T> True<T>
where
    T: LanguageTerm,
{
    pub fn new() -> True<T> {
        True {
            phantom: PhantomData,
        }
    }
}

impl<T> Default for True<T>
where
    T: LanguageTerm,
{
    fn default() -> True<T> {
        True::new()
    }
}

impl<T> Term for True<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for True<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for True<T>
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

impl<T> Typecheck for True<T>
where
    T: LanguageTerm,
    Bool<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Bool::new().into())
    }
}

impl<T> Eval for True<T>
where
    T: LanguageTerm,
    TrueVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(TrueVal::new().into())
    }
}

impl<T> fmt::Display for True<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
