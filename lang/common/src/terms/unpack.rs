use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar, Var};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Unpack<T>
where
    T: Term,
{
    ty_name: TypeVar,
    term_name: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Term for Unpack<T> where T: Term {}

impl<T, Ty> SubstType<Ty> for Unpack<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        let bound_subst = self.bound_term.subst_type(v, ty);
        if *v == self.ty_name {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(bound_subst),
                in_term: self.in_term,
            }
            .into()
        } else {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(bound_subst),
                in_term: Box::new(self.in_term.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<T> fmt::Display for Unpack<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {{{},{}}}={} in {}",
            self.ty_name, self.term_name, self.bound_term, self.in_term
        )
    }
}
