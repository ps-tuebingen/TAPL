use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Kindcheck, Typecheck},
    errors::{Error, ErrorKind},
    eval::{Eval, Normalize},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SomeCase<T>
where
    T: LanguageTerm,
{
    bound_term: Box<T>,
    none_term: Box<T>,
    some_var: Var,
    some_term: Box<T>,
}

impl<T> SomeCase<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, T2, T3>(bound: T1, none: T2, v: &str, some: T3) -> SomeCase<T>
    where
        T1: Into<T>,
        T2: Into<T>,
        T3: Into<T>,
    {
        SomeCase {
            bound_term: Box::new(bound.into()),
            none_term: Box::new(none.into()),
            some_var: v.to_owned(),
            some_term: Box::new(some.into()),
        }
    }
}

impl<T> Term for SomeCase<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for SomeCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.some_var {
            SomeCase {
                bound_term: Box::new(self.bound_term.subst(v, t)),
                none_term: Box::new(self.none_term.subst(v, t)),
                some_var: self.some_var,
                some_term: self.some_term,
            }
            .into()
        } else {
            SomeCase {
                bound_term: Box::new(self.bound_term.subst(v, t)),
                none_term: Box::new(self.none_term.subst(v, t)),
                some_var: self.some_var,
                some_term: Box::new(self.some_term.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for SomeCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        SomeCase {
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            none_term: Box::new(self.none_term.subst_type(v, ty)),
            some_var: self.some_var,
            some_term: Box::new(self.some_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for SomeCase<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?.normalize(env);
        bound_ty
            .check_kind(env)?
            .into_star()
            .map_err(to_check_err)?;

        let option = bound_ty.into_optional().map_err(to_check_err)?;
        let mut some_env = env.clone();
        some_env.add_var(self.some_var.clone(), *option.ty);
        let some_ty = self.some_term.check(&mut some_env)?.normalize(env);
        let some_knd = some_ty.check_kind(&mut some_env)?;

        let none_ty = self.none_term.check(env)?.normalize(env);
        let none_knd = none_ty.check_kind(env)?;

        some_knd.check_equal(&none_knd).map_err(to_check_err)?;
        some_ty.check_equal(&none_ty).map_err(to_check_err)?;
        Ok(some_ty)
    }
}

impl<T> Eval for SomeCase<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(env)?;

        if let Ok(some_val) = bound_val.clone().into_something() {
            self.some_term
                .subst(&self.some_var, &((*some_val.val).into()))
                .eval(env)
        } else if let Ok(_) = bound_val.clone().into_nothing() {
            self.none_term.eval(env)
        } else {
            Err(to_check_err(ErrorKind::ValueMismatch {
                found: bound_val.to_string(),
                expected: "Option Value".to_owned(),
            }))
        }
    }
}
impl<T> fmt::Display for SomeCase<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ Nothing => {} | Something({}) => {} }}",
            self.bound_term, self.none_term, self.some_var, self.some_term
        )
    }
}
