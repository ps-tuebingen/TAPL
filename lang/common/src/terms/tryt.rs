use super::Term;
use crate::{
    check::{to_check_err, Typecheck},
    errors::{Error, ErrorKind},
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Try<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> Try<T>
where
    T: LanguageTerm,
{
    pub fn new<T1: Into<T>, T2: Into<T>>(t: T1, h: T2) -> Try<T> {
        Try {
            term: Box::new(t.into()),
            handler: Box::new(h.into()),
        }
    }
}

impl<T> Term for Try<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Try<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Try {
            term: Box::new(self.term.subst(v, t)),
            handler: Box::new(self.handler.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Try<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Try {
            term: Box::new(self.term.subst_type(v, ty)),
            handler: Box::new(self.handler.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Try<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.term.check(&mut env.clone())?;
        let handler_ty = self.handler.check(env)?;
        if term_ty == handler_ty {
            Ok(term_ty)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: term_ty.to_string(),
                expected: handler_ty.to_string(),
            }))
        }
    }
}

impl<T> Eval for Try<T>
where
    T: LanguageTerm,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env);
        if term_val.is_err() {
            self.handler.eval(env)
        } else {
            term_val
        }
    }
}

impl<T> fmt::Display for Try<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} with {{ {} }}", self.term, self.handler)
    }
}
