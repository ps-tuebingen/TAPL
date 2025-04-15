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
pub struct TryWithVal<T>
where
    T: Term,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> Term for TryWithVal<T> where T: Term {}

impl<T> SubstTerm<T> for TryWithVal<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TryWithVal {
            term: Box::new(self.term.subst(v, t)),
            handler: Box::new(self.handler.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for TryWithVal<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        TryWithVal {
            term: Box::new(self.term.subst_type(v, ty)),
            handler: Box::new(self.handler.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Env, Ty, T> Typecheck<Env, Ty> for TryWithVal<T>
where
    T: Term + Typecheck<Env, Ty>,
    Ty: Type,
    Env: CheckEnvironment<Ty>,
{
    fn check(&self, env: &mut Env) -> Result<Ty, Error> {
        let t_ty = self.term.check(&mut env.clone())?;
        let handler_ty = self.handler.check(env)?;
        let fun = handler_ty.into_fun().map_err(to_check_err)?;
        if t_ty == *fun.to {
            Ok(t_ty)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: t_ty.to_string(),
                expected: fun.to.to_string(),
            }))
        }
    }
}

impl<T> fmt::Display for TryWithVal<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} catch {{ {} }}", self.term, self.handler)
    }
}
