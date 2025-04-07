use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct True;

#[derive(Debug, Clone)]
pub struct False;

#[derive(Debug, Clone)]
pub struct If {
    pub ifc: Box<Term>,
    pub thent: Box<Term>,
    pub elset: Box<Term>,
}

impl SubstTerm for True {
    fn subst(self, _: &Var, _: Term) -> Term {
        True.into()
    }
}

impl SubstTerm for False {
    fn subst(self, _: &Var, _: Term) -> Term {
        False.into()
    }
}

impl SubstTerm for If {
    fn subst(self, v: &Var, t: Term) -> Term {
        If {
            ifc: Box::new(self.ifc.subst(v, t.clone())),
            thent: Box::new(self.thent.subst(v, t.clone())),
            elset: Box::new(self.elset.subst(v, t)),
        }
        .into()
    }
}

impl SubstTy for True {
    fn subst_ty(self, _: &TypeVar, _: Type) -> Self {
        self
    }
}
impl SubstTy for False {
    fn subst_ty(self, _: &TypeVar, _: Type) -> Self {
        self
    }
}
impl SubstTy for If {
    fn subst_ty(self, var: &TypeVar, ty: Type) -> Self {
        If {
            ifc: Box::new(self.ifc.subst_ty(var, ty.clone())),
            thent: Box::new(self.thent.subst_ty(var, ty.clone())),
            elset: Box::new(self.elset.subst_ty(var, ty.clone())),
        }
    }
}

impl From<True> for Term {
    fn from(tru: True) -> Term {
        Term::True(tru)
    }
}

impl From<False> for Term {
    fn from(fls: False) -> Term {
        Term::False(fls)
    }
}

impl From<If> for Term {
    fn from(ift: If) -> Term {
        Term::If(ift)
    }
}

impl fmt::Display for True {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
impl fmt::Display for False {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}
impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.ifc, self.thent, self.elset
        )
    }
}
