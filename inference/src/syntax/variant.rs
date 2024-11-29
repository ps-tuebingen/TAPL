use super::Term;
use crate::Label;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub label: Label,
    pub term: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantPattern {
    pub label: Label,
    pub bound_var: Label,
    pub rhs: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
        write!(f, "<{}={}>", self.label, self.term)
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
