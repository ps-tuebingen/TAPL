use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval},
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable<T>
where
    T: LanguageTerm,
{
    var: Var,
    phantom: PhantomData<T>,
}

impl<T: LanguageTerm> Variable<T> {
    pub fn new(v: &str) -> Variable<T> {
        Variable {
            var: v.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Variable<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Variable<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            t.clone()
        }
    }
}

impl<T, Ty> SubstType<Ty> for Variable<T>
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

impl<T> Typecheck for Variable<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        env.get_var(&self.var).map_err(to_check_err)
    }
}

impl<T> Eval for Variable<T>
where
    T: LanguageTerm,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Err(to_eval_err(ErrorKind::FreeVariable(self.var)))
    }
}

impl<T> fmt::Display for Variable<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}
