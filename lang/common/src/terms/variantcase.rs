use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    Label, TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantCase<T>
where
    T: LanguageTerm,
{
    bound_term: Box<T>,
    patterns: Vec<VariantPattern<T>>,
}

impl<T> VariantCase<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(bound: T1, pts: Vec<VariantPattern<T>>) -> VariantCase<T>
    where
        T1: Into<T>,
    {
        VariantCase {
            bound_term: Box::new(bound.into()),
            patterns: pts,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantPattern<T>
where
    T: LanguageTerm,
{
    label: Label,
    bound_var: Var,
    rhs: Box<T>,
}

impl<T> VariantPattern<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(lb: &str, bound: &str, rhs: T1) -> VariantPattern<T>
    where
        T1: Into<T>,
    {
        VariantPattern {
            label: lb.to_owned(),
            bound_var: bound.to_owned(),
            rhs: Box::new(rhs.into()),
        }
    }
}

impl<T> Term for VariantCase<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for VariantCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        VariantCase {
            bound_term: Box::new(self.bound_term.subst(v, t)),
            patterns: self.patterns.into_iter().map(|pt| pt.subst(v, t)).collect(),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for VariantCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        VariantCase {
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            patterns: self
                .patterns
                .into_iter()
                .map(|pt| pt.subst_type(v, ty))
                .collect(),
        }
        .into()
    }
}

impl<T> SubstTerm<T> for VariantPattern<T>
where
    T: LanguageTerm,
{
    type Target = Self;
    fn subst(self, v: &Var, t: &T) -> Self::Target {
        if *v == self.bound_var {
            VariantPattern {
                label: self.label,
                bound_var: self.bound_var,
                rhs: self.rhs,
            }
        } else {
            VariantPattern {
                label: self.label,
                bound_var: self.bound_var,
                rhs: Box::new(self.rhs.subst(v, t)),
            }
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for VariantPattern<T>
where
    T: LanguageTerm,
{
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        VariantPattern {
            label: self.label,
            bound_var: self.bound_var,
            rhs: Box::new(self.rhs.subst_type(v, ty)),
        }
    }
}

impl<T> fmt::Display for VariantCase<T>
where
    T: LanguageTerm,
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
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> => {}", self.label, self.bound_var, self.rhs)
    }
}
