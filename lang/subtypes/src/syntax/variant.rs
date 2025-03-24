use super::{Label, Term, Var};
use crate::types::Type;
use std::fmt;

#[derive(Debug)]
pub struct Variant {
    pub label: Label,
    pub term: Box<Term>,
}

impl Variant {
    pub fn new(l: &str, t: Term) -> Variant {
        Variant {
            label: l.to_owned(),
            term: Box::new(t),
        }
    }
}

#[derive(Debug)]
pub struct VariantPattern {
    pub label: Label,
    pub bound_var: Var,
    pub var_ty: Type,
    pub rhs: Box<Term>,
}

impl VariantPattern {
    pub fn new(label: &str, var: &str, var_ty: Type, rhs: Term) -> VariantPattern {
        VariantPattern {
            label: label.to_owned(),
            bound_var: var.to_owned(),
            var_ty,
            rhs: Box::new(rhs),
        }
    }
}

#[derive(Debug)]
pub struct VariantCase {
    pub bound_term: Box<Term>,
    pub patterns: Vec<VariantPattern>,
}

impl VariantCase {
    pub fn new(t: Term, pts: Vec<VariantPattern>) -> VariantCase {
        VariantCase {
            bound_term: Box::new(t),
            patterns: pts,
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
        write!(f, "<{}={}>", self.label, self.term)
    }
}

impl fmt::Display for VariantPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{}={}:{}> => {}",
            self.label, self.bound_var, self.var_ty, self.rhs
        )
    }
}

impl fmt::Display for VariantCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pt_strs: Vec<String> = self.patterns.iter().map(|pt| format!("{pt}")).collect();
        write!(
            f,
            "case {} of {{ {} }}",
            self.bound_term,
            pt_strs.join("| ")
        )
    }
}
