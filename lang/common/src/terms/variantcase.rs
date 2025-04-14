use super::Term;
use crate::{Label, Var};
use std::fmt;

#[derive(Clone, Debug)]
pub struct VariantCase<T>
where
    T: Term,
{
    bound_term: Box<T>,
    patterns: Vec<VariantPattern<T>>,
}

#[derive(Clone, Debug)]
pub struct VariantPattern<T>
where
    T: Term,
{
    label: Label,
    bound_var: Var,
    rhs: Box<T>,
}

impl<T> Term for VariantCase<T> where T: Term {}

impl<T> fmt::Display for VariantCase<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pts: Vec<&VariantPattern<T>> = self.patterns.iter().collect();
        pts.sort_by(|pt1, pt2| pt1.label.cmp(&pt2.label));

        write!(
            f,
            "case {} of {{ {} }}",
            self.bound_term,
            pts.iter()
                .map(|pt| pt.to_string())
                .collect::<Vec<String>>()
                .join("| ")
        )
    }
}

impl<T> fmt::Display for VariantPattern<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> => {}", self.label, self.bound_var, self.rhs)
    }
}
