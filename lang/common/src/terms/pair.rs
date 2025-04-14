use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Pair<T>
where
    T: Term,
{
    fst: Box<T>,
    snd: Box<T>,
}

impl<T> Term for Pair<T> where T: Term {}

impl<T> SubstTerm<T> for Pair<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Pair {
            fst: Box::new(self.fst.subst(v, t)),
            snd: Box::new(self.snd.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Pair<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Pair {
            fst: Box::new(self.fst.subst_type(v, ty)),
            snd: Box::new(self.snd.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Pair<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
