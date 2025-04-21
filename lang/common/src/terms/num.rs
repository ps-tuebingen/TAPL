use super::Term;
use crate::{
    check::Typecheck,
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Nat,
    values::Num as NumVal,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Num<T>
where
    T: LanguageTerm,
{
    num: i64,
    phantom: PhantomData<T>,
}

impl<T> Num<T>
where
    T: LanguageTerm,
{
    pub fn new(num: i64) -> Num<T> {
        Num {
            num,
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Num<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Num<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Num<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &<T as LanguageTerm>::Type) -> Self::Target {
        self.into()
    }
}

impl<T> Typecheck for Num<T>
where
    T: LanguageTerm,
    Nat<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Nat::new().into())
    }
}

impl<T> Eval for Num<T>
where
    T: LanguageTerm,
    NumVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NumVal::new(self.num).into())
    }
}

impl<T> fmt::Display for Num<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
