use super::Term;
use crate::{
    kinds::Kind,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyLambda<T>
where
    T: Term,
{
    var: TypeVar,
    annot: Kind,
    term: Box<T>,
}

impl<T> TyLambda<T>
where
    T: Term,
{
    pub fn new<T1>(v: &str, knd: Kind, t: T1) -> TyLambda<T>
    where
        T1: Into<T>,
    {
        TyLambda {
            var: v.into(),
            annot: knd,
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for TyLambda<T> where T: Term {}

impl<T> SubstTerm<T> for TyLambda<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TyLambda {
            var: self.var,
            annot: self.annot,
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for TyLambda<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            TyLambda {
                var: self.var,
                annot: self.annot,
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
        write!(f, "\\{}::{}.({})", self.var, self.annot, self.term)
    }
}
