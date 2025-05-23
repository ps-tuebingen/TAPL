use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tuple<T>
where
    T: Term,
{
    terms: Vec<T>,
}

impl<T> Tuple<T>
where
    T: Term,
{
    pub fn new<T1>(ts: Vec<T1>) -> Tuple<T>
    where
        T1: Into<T>,
    {
        Tuple {
            terms: ts.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl<T> Term for Tuple<T> where T: Term {}

impl<T> SubstTerm<T> for Tuple<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Tuple {
            terms: self.terms.into_iter().map(|t1| t1.subst(v, t)).collect(),
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Tuple<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Tuple {
            terms: self
                .terms
                .into_iter()
                .map(|t| t.subst_type(v, ty))
                .collect(),
        }
        .into()
    }
}

impl<T> fmt::Display for Tuple<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self.terms.iter().map(|t| t.to_string()).collect();
        ts.sort();
        write!(f, "{{ {} }}", ts.join(", "))
    }
}
