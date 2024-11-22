use super::Subst;
use crate::{
    terms::syntax::{Cons, Head, IsNil, Nil, Tail, Term},
    Var,
};

impl Subst for Nil {
    type Target = Nil;
    fn subst(self, _: Var, _: Term) -> Self::Target {
        self
    }
}

impl Subst for Cons {
    type Target = Cons;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Cons {
            inner_type: self.inner_type,
            fst: self.fst.subst(var.clone(), term.clone()),
            rst: self.rst.subst(var, term),
        }
    }
}

impl Subst for IsNil {
    type Target = IsNil;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        IsNil {
            inner_type: self.inner_type,
            list: self.list.subst(var, term),
        }
    }
}

impl Subst for Tail {
    type Target = Tail;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Tail {
            inner_type: self.inner_type,
            list: self.list.subst(var, term),
        }
    }
}

impl Subst for Head {
    type Target = Head;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Head {
            inner_type: self.inner_type,
            list: self.list.subst(var, term),
        }
    }
}
