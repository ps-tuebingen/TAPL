use crate::{
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    language::{LanguageTerm, LanguageType},
    subst::SubstType,
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Untyped;

impl Type for Untyped {}
impl LanguageType for Untyped {}
impl SubstType<Untyped> for Untyped {
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Self) -> Self::Target {
        self
    }
}

impl fmt::Display for Untyped {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("untyped")
    }
}

impl CheckEnvironment for () {
    type Type = Untyped;
    fn get_var(&self, v: &Var) -> Result<Self::Type, Error> {
        Err(to_check_err(ErrorKind::FreeVariable(v.clone())))
    }

    fn add_var(&mut self, _: Var, _: Untyped) {}
}

impl<T> Typecheck for T
where
    T: LanguageTerm<Type = Untyped>,
{
    type Type = Untyped;
    type Env = ();

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Untyped)
    }
}

impl<T> SubstType<Untyped> for T
where
    T: LanguageTerm<Type = Untyped>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &Untyped) -> Self::Target {
        self
    }
}
