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
pub struct True;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct False;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    pub ifc: Box<Term>,
    pub thenc: Box<Term>,
    pub elsec: Box<Term>,
}

impl If {
    pub fn new(ifc: Term, thenc: Term, elsec: Term) -> If {
        If {
            ifc: Box::new(ifc),
            thenc: Box::new(thenc),
            elsec: Box::new(elsec),
        }
    }
}

impl IsValue for True {
    fn is_value(&self) -> bool {
        true
    }
}

impl IsValue for False {
    fn is_value(&self) -> bool {
        true
    }
}

impl IsValue for If {
    fn is_value(&self) -> bool {
        false
    }
}

impl SubstTy for True {
    fn subst_ty(self, _: TypeVar, _: Type) -> Self {
        True
    }
}

impl SubstTy for False {
    fn subst_ty(self, _: TypeVar, _: Type) -> Self {
        False
    }
}

impl SubstTy for If {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        If {
            ifc: Box::new(self.ifc.subst_ty(v.clone(), ty.clone())),
            thenc: Box::new(self.thenc.subst_ty(v.clone(), ty.clone())),
            elsec: Box::new(self.elsec.subst_ty(v, ty)),
        }
    }
}

impl SubstTerm for True {
    fn subst(self, _: Var, _: Term) -> Self {
        True
    }
}

impl SubstTerm for False {
    fn subst(self, _: Var, _: Term) -> Self {
        False
    }
}

impl SubstTerm for If {
    fn subst(self, v: Var, t: Term) -> Self {
        If {
            ifc: Box::new(self.ifc.subst(v.clone(), t.clone())),
            thenc: Box::new(self.thenc.subst(v.clone(), t.clone())),
            elsec: Box::new(self.elsec.subst(v, t)),
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
        f.write_str("True")
    }
}

impl fmt::Display for False {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("False")
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "If {} then {} else {}", self.ifc, self.thenc, self.elsec)
    }
}
