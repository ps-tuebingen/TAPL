use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval, Normalize},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Projection<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
    index: usize,
}

impl<T> Projection<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(t: T1, ind: usize) -> Projection<T>
    where
        T1: Into<T>,
    {
        Projection {
            term: Box::new(t.into()),
            index: ind,
        }
    }
}

impl<T> Term for Projection<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Projection<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Projection {
            term: Box::new(self.term.subst(v, t)),
            index: self.index,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Projection<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Projection {
            term: Box::new(self.term.subst_type(v, ty)),
            index: self.index,
        }
        .into()
    }
}

impl<T> Typecheck for Projection<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.term.check(env)?.normalize(env);
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let tup_ty = term_ty.into_tuple().map_err(to_check_err)?;
        tup_ty
            .tys
            .get(self.index)
            .ok_or(to_check_err(ErrorKind::Arity {
                found: tup_ty.tys.len(),
                expected: self.index,
            }))
            .cloned()
    }
}

impl<T> Eval for Projection<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let tup_val = term_val.into_tuple().map_err(to_eval_err)?;
        tup_val
            .vals
            .get(self.index)
            .ok_or(to_eval_err(ErrorKind::Arity {
                found: tup_val.vals.len(),
                expected: self.index,
            }))
            .cloned()
    }
}

impl<T> fmt::Display for Projection<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).{}", self.term, self.index)
    }
}
