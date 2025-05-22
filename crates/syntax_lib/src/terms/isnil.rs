use super::Term;
use common::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval, Normalize},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    types::{Bool, List},
    values::{False, True},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IsNil<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> IsNil<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty>(t: T1, ty: Ty) -> IsNil<T>
    where
        T1: Into<T>,
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        IsNil {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for IsNil<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for IsNil<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        IsNil {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for IsNil<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        IsNil {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for IsNil<T>
where
    T: LanguageTerm,
    List<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
    Bool<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        term_ty.into_list().map_err(to_check_err)?;
        Ok(Bool::new().into())
    }
}

impl<T> Eval for IsNil<T>
where
    T: LanguageTerm,
    True<T>: Into<<T as LanguageTerm>::Value>,
    False<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        if term_val.clone().into_nil().is_ok() {
            Ok(True::new().into())
        } else if term_val.clone().into_cons().is_ok() {
            Ok(False::new().into())
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: term_val.to_string(),
                expected: "List Value".to_owned(),
            }))
        }
    }
}

impl<T> fmt::Display for IsNil<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "isnil[{}]({})", self.ty, self.term)
    }
}
