use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UntypedLambda<T>
where
    T: Term,
{
    pub var: Var,
    pub body: Box<T>,
}

impl<T> UntypedLambda<T>
where
    T: Term,
{
    pub fn new<T1>(v: &str, t: T1) -> UntypedLambda<T>
    where
        T1: Into<T>,
    {
        UntypedLambda {
            var: v.to_owned(),
            body: Box::new(t.into()),
        }
    }
}

impl<T> Term for UntypedLambda<T> where T: Term {}

impl<T> SubstTerm<T> for UntypedLambda<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            UntypedLambda {
                var: self.var,
                body: Box::new(self.body.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T, Ty> SubstType<Ty> for UntypedLambda<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        UntypedLambda {
            var: self.var,
            body: Box::new(self.body.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for UntypedLambda<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}.{}", self.var, self.body)
    }
}
