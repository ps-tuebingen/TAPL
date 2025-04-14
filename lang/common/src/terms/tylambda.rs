use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct TyLambda<T>
where
    T: Term,
{
    var: TypeVar,
    term: Box<T>,
}

impl<T> Term for TyLambda<T> where T: Term {}

impl<T> SubstTerm<T> for TyLambda<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TyLambda {
            var: self.var,
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for TyLambda<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            TyLambda {
                var: self.var,
                term: Box::new(self.term.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<T> fmt::Display for TyLambda<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}.{}", self.var, self.term)
    }
}
