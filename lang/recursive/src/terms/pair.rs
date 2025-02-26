use super::{Term, Var};
use crate::{
    traits::{
        is_value::IsValue,
        subst::{SubstTerm, SubstTy},
    },
    types::{Type, TypeVar},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    pub fst: Box<Term>,
    pub snd: Box<Term>,
}

impl Pair {
    pub fn new(fst: Term, snd: Term) -> Pair {
        Pair {
            fst: Box::new(fst),
            snd: Box::new(snd),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fst {
    pub term: Box<Term>,
}

impl Fst {
    pub fn new(t: Term) -> Fst {
        Fst { term: Box::new(t) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snd {
    pub term: Box<Term>,
}

impl Snd {
    pub fn new(t: Term) -> Snd {
        Snd { term: Box::new(t) }
    }
}

impl IsValue for Pair {
    fn is_value(&self) -> bool {
        self.fst.is_value() && self.snd.is_value()
    }
}

impl IsValue for Fst {
    fn is_value(&self) -> bool {
        false
    }
}

impl IsValue for Snd {
    fn is_value(&self) -> bool {
        false
    }
}

impl SubstTy for Pair {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Pair {
            fst: Box::new(self.fst.subst_ty(v.clone(), ty.clone())),
            snd: Box::new(self.snd.subst_ty(v, ty)),
        }
    }
}
impl SubstTy for Fst {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Fst {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}
impl SubstTy for Snd {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Snd {
            term: Box::new(self.term.subst_ty(v, ty)),
        }
    }
}

impl SubstTerm for Pair {
    fn subst(self, v: Var, t: Term) -> Self {
        Pair {
            fst: Box::new(self.fst.subst(v.clone(), t.clone())),
            snd: Box::new(self.snd.subst(v, t)),
        }
    }
}

impl SubstTerm for Fst {
    fn subst(self, v: Var, t: Term) -> Self {
        Fst {
            term: Box::new(self.term.subst(v, t)),
        }
    }
}

impl SubstTerm for Snd {
    fn subst(self, v: Var, t: Term) -> Self {
        Snd {
            term: Box::new(self.term.subst(v, t)),
        }
    }
}

impl From<Pair> for Term {
    fn from(p: Pair) -> Term {
        Term::Pair(p)
    }
}

impl From<Fst> for Term {
    fn from(fst: Fst) -> Term {
        Term::Fst(fst)
    }
}

impl From<Snd> for Term {
    fn from(snd: Snd) -> Term {
        Term::Snd(snd)
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{},{}}}", self.fst, self.snd)
    }
}

impl fmt::Display for Fst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.0", self.term)
    }
}

impl fmt::Display for Snd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.1", self.term)
    }
}
