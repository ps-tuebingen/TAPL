use super::Term;
use crate::{types::Type, Var};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Variant {
    pub label: Var,
    pub term: Box<Term>,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct VariantPattern {
    pub label: Var,
    pub bound_var: Var,
    pub rhs: Box<Term>,
}

#[derive(Debug, Clone)]
pub struct VariantCase {
    pub bound_term: Box<Term>,
    pub cases: Vec<VariantPattern>,
}

impl From<VariantCase> for Term {
    fn from(case: VariantCase) -> Term {
        Term::VariantCase(case)
    }
}

impl From<Variant> for Term {
    fn from(var: Variant) -> Term {
        Term::Variant(var)
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}={}> as {}", self.label, self.term, self.ty)
    }
}

impl fmt::Display for VariantPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}={}>=>{}", self.label, self.bound_var, self.rhs)
    }
}

impl fmt::Display for VariantCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {}",
            self.bound_term,
            self.cases
                .iter()
                .map(|case| format!("{case}"))
                .collect::<Vec<String>>()
                .join(" | ")
        )
    }
}
