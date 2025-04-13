use super::{Term, Var};
use crate::{
    traits::{
        is_value::IsValue,
        subst::{SubstTerm, SubstTy},
    },
    types::{Type, TypeVar},
};
use common::Label;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub label: Label,
    pub term: Box<Term>,
    pub annot: Type,
}

impl Variant {
    pub fn new(label: &str, term: Term, annot: Type) -> Variant {
        Variant {
            label: label.to_owned(),
            term: Box::new(term),
            annot,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
    pub bound_term: Box<Term>,
    pub patterns: Vec<VariantPattern>,
}

impl VariantCase {
    pub fn new(bound: Term, patterns: Vec<VariantPattern>) -> VariantCase {
        VariantCase {
            bound_term: Box::new(bound),
            patterns,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantPattern {
    pub label: Label,
    pub bound_var: Var,
    pub rhs: Box<Term>,
}

impl VariantPattern {
    pub fn new(label: &str, var: &str, rhs: Term) -> VariantPattern {
        VariantPattern {
            label: label.to_owned(),
            bound_var: var.to_owned(),
            rhs: Box::new(rhs),
        }
    }
}

impl IsValue for Variant {
    fn is_value(&self) -> bool {
        self.term.is_value()
    }
}

impl IsValue for VariantCase {
    fn is_value(&self) -> bool {
        false
    }
}

impl SubstTy for Variant {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Variant {
            label: self.label,
            term: Box::new(self.term.subst_ty(v.clone(), ty.clone())),
            annot: self.annot.subst_ty(v, ty),
        }
    }
}

impl SubstTy for VariantCase {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        VariantCase {
            patterns: self
                .patterns
                .into_iter()
                .map(|pt| pt.subst_ty(v.clone(), ty.clone()))
                .collect(),
            bound_term: Box::new(self.bound_term.subst_ty(v, ty)),
        }
    }
}

impl SubstTy for VariantPattern {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        VariantPattern {
            label: self.label,
            bound_var: self.bound_var,
            rhs: Box::new(self.rhs.subst_ty(v, ty)),
        }
    }
}

impl SubstTerm for Variant {
    fn subst(self, v: Var, t: Term) -> Self {
        Variant {
            label: self.label,
            term: Box::new(self.term.subst(v, t)),
            annot: self.annot,
        }
    }
}

impl SubstTerm for VariantCase {
    fn subst(self, v: Var, t: Term) -> Self {
        VariantCase {
            patterns: self
                .patterns
                .into_iter()
                .map(|pt| pt.subst(v.clone(), t.clone()))
                .collect(),
            bound_term: Box::new(self.bound_term.subst(v, t)),
        }
    }
}

impl SubstTerm for VariantPattern {
    fn subst(self, v: Var, t: Term) -> Self {
        if v == self.bound_var {
            self
        } else {
            VariantPattern {
                label: self.label,
                bound_var: self.bound_var,
                rhs: Box::new(self.rhs.subst(v, t)),
            }
        }
    }
}

impl From<Variant> for Term {
    fn from(var: Variant) -> Term {
        Term::Variant(var)
    }
}

impl From<VariantCase> for Term {
    fn from(case: VariantCase) -> Term {
        Term::VariantCase(case)
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> as {}", self.label, self.term, self.annot)
    }
}

impl fmt::Display for VariantCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ {} }}",
            self.bound_term,
            self.patterns
                .iter()
                .map(|pt| pt.to_string())
                .collect::<Vec<String>>()
                .join("| ")
        )
    }
}
impl fmt::Display for VariantPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}={}> => {}", self.label, self.bound_var, self.rhs)
    }
}
