use super::Term;
use common::{
    check::{CheckEnvironment, Kindcheck, Typecheck},
    errors::Error,
    eval::{Eval, Normalize},
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let<T>
where
    T: LanguageTerm,
{
    var: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Let<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, T2>(v: &str, bound: T1, int: T2) -> Let<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        Let {
            var: v.to_owned(),
            bound_term: Box::new(bound.into()),
            in_term: Box::new(int.into()),
        }
    }
}
impl<T> Term for Let<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Let<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            Let {
                var: self.var,
                bound_term: Box::new(self.bound_term.subst(v, t)),
                in_term: Box::new(self.in_term.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Let<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Let {
            var: self.var,
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            in_term: Box::new(self.in_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Let<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        bound_ty.check_kind(&mut env.clone())?;

        env.add_var(self.var.clone(), bound_ty);
        let in_ty = self
            .in_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        in_ty.check_kind(env)?;
        Ok(in_ty)
    }
}

impl<T> Eval for Let<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(env)?;
        self.in_term.subst(&self.var, &bound_val.into()).eval(env)
    }
}

impl<T> fmt::Display for Let<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let ({} = {}) in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}
