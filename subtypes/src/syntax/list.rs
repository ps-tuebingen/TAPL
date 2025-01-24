use super::{Term, Var};
use crate::types::Type;
use std::fmt;

pub struct Nil {
    pub ty: Type,
}

pub struct Cons {
    pub fst: Box<Term>,
    pub rst: Box<Term>,
}

impl Cons {
    pub fn new(fst: Term, rst: Term) -> Cons {
        Cons {
            fst: Box::new(fst),
            rst: Box::new(rst),
        }
    }
}

pub struct ListCase {
    pub bound_term: Box<Term>,
    pub list_ty: Type,
    pub nil_rhs: Box<Term>,
    pub cons_fst: Var,
    pub cons_rst: Var,
    pub cons_rhs: Box<Term>,
}

impl ListCase {
    pub fn new(
        bound: Term,
        list_ty: Type,
        nil_rhs: Term,
        fst: &str,
        rst: &str,
        cons_rhs: Term,
    ) -> ListCase {
        ListCase {
            bound_term: Box::new(bound),
            list_ty,
            nil_rhs: Box::new(nil_rhs),
            cons_fst: fst.to_owned(),
            cons_rst: rst.to_owned(),
            cons_rhs: Box::new(cons_rhs),
        }
    }
}

impl From<Nil> for Term {
    fn from(nil: Nil) -> Term {
        Term::Nil(nil)
    }
}

impl From<Cons> for Term {
    fn from(cons: Cons) -> Term {
        Term::Cons(cons)
    }
}

impl From<ListCase> for Term {
    fn from(case: ListCase) -> Term {
        Term::ListCase(case)
    }
}

impl fmt::Display for Nil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nil[{}]", self.ty)
    }
}

impl fmt::Display for Cons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons({},{})", self.fst, self.rst)
    }
}

impl fmt::Display for ListCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ Nil => {}, Cons({},{}) => {} }}",
            self.bound_term, self.nil_rhs, self.cons_fst, self.cons_rst, self.cons_rhs
        )
    }
}
