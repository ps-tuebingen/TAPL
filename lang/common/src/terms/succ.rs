use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::Error,
    subst::{SubstTerm, SubstType},
    types::{Nat, Type},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Succ<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Succ<T> where T: Term {}

impl<T> SubstTerm<T> for Succ<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Succ {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Succ<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Succ {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Env, Ty, T> Typecheck<Env, Ty> for Succ<T>
where
    Env: CheckEnvironment<Ty>,
    Ty: Type + Typecheck<Env, Ty>,
    T: Term + Typecheck<Env, Ty>,
    Nat: Into<Ty>,
{
    fn check(&self, env: &mut Env) -> Result<Ty, Error> {
        let inner_ty = self.term.check(env)?;
        let nat = inner_ty.into_nat().map_err(to_check_err)?;
        Ok(nat.into())
    }
}

impl<T> fmt::Display for Succ<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}
