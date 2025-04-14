use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Tuple<T>
where
    T: Term,
{
    terms: Vec<T>,
}

impl<T> Term for Tuple<T> where T: Term {}

impl<T, Ty> SubstType<Ty> for Tuple<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
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
        write!(f, "( {} )", ts.join(", "))
    }
}
