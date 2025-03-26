use super::{Term, Var};
use crate::types::{Type, TypeVar};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Pack {
    pub inner_ty: Type,
    pub term: Box<Term>,
    pub outer_ty: Type,
}

impl Pack {
    pub fn new<T: Into<Term>>(inner: Type, term: T, outer: Type) -> Pack {
        Pack {
            inner_ty: inner,
            term: Box::new(term.into()),
            outer_ty: outer,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unpack {
    pub ty_var: TypeVar,
    pub bound_var: Var,
    pub bound_term: Box<Term>,
    pub in_term: Box<Term>,
}

impl Unpack {
    pub fn new<T: Into<Term>, U: Into<Term>>(tyvar: &str, var: &str, bound: T, int: U) -> Unpack {
        Unpack {
            ty_var: tyvar.to_owned(),
            bound_var: var.to_owned(),
            bound_term: Box::new(bound.into()),
            in_term: Box::new(int.into()),
        }
    }
}

impl From<Pack> for Term {
    fn from(pack: Pack) -> Term {
        Term::Pack(pack)
    }
}

impl From<Unpack> for Term {
    fn from(unpack: Unpack) -> Term {
        Term::Unpack(unpack)
    }
}

impl fmt::Display for Pack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*{},{}}} as {{ {} }}",
            self.inner_ty, self.term, self.outer_ty
        )
    }
}
impl fmt::Display for Unpack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {{{},{}}}={} in {}",
            self.ty_var, self.bound_var, self.bound_term, self.in_term
        )
    }
}
