use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If<T>
where
    T: Term,
{
    if_cond: Box<T>,
    then_term: Box<T>,
    else_term: Box<T>,
}

impl<T> Term for If<T> where T: Term {}

impl<T> SubstTerm<T> for If<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        If {
            if_cond: Box::new(self.if_cond.subst(v, t)),
            then_term: Box::new(self.then_term.subst(v, t)),
            else_term: Box::new(self.else_term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for If<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        If {
            if_cond: Box::new(self.if_cond.subst_type(v, ty)),
            then_term: Box::new(self.then_term.subst_type(v, ty)),
            else_term: Box::new(self.else_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Env, Ty, T> Typecheck<Env, Ty> for If<T>
where
    T: Term + Typecheck<Env, Ty>,
    Ty: Type,
    Env: CheckEnvironment<Ty>,
{
    fn check_start(&self) -> Result<Ty, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: &mut Env) -> Result<Ty, Error> {
        let if_ty = self.if_cond.check(&mut env.clone())?;
        if_ty.into_bool().map_err(to_check_err)?;
        let then_ty = self.then_term.check(&mut env.clone())?;
        let else_ty = self.else_term.check(env)?;
        if then_ty != else_ty {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: then_ty.to_string(),
                expected: else_ty.to_string(),
            }))
        } else {
            Ok(then_ty)
        }
    }
}

impl<T> fmt::Display for If<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.if_cond, self.then_term, self.else_term
        )
    }
}
