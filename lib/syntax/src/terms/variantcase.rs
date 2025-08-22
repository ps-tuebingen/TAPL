use super::Term;
use crate::{
    Label, TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantCase<Lang>
where
    Lang: Language,
{
    pub bound_term: Box<Lang::Term>,
    pub patterns: Vec<VariantPattern<Lang>>,
}

impl<Lang> VariantCase<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(bound: T1, pts: Vec<VariantPattern<Lang>>) -> VariantCase<Lang>
    where
        T1: Into<Lang::Term>,
    {
        VariantCase {
            bound_term: Box::new(bound.into()),
            patterns: pts,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantPattern<Lang>
where
    Lang: Language,
{
    pub label: Label,
    pub bound_var: Var,
    pub rhs: Box<Lang::Term>,
}

impl<Lang> VariantPattern<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(lb: &str, bound: &str, rhs: T1) -> VariantPattern<Lang>
    where
        T1: Into<Lang::Term>,
    {
        VariantPattern {
            label: lb.to_owned(),
            bound_var: bound.to_owned(),
            rhs: Box::new(rhs.into()),
        }
    }
}

impl<Lang> Term for VariantCase<Lang> where Lang: Language {}

impl<Lang> SubstTerm for VariantCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        VariantCase {
            bound_term: Box::new(self.bound_term.subst(v, t)),
            patterns: self.patterns.into_iter().map(|pt| pt.subst(v, t)).collect(),
        }
        .into()
    }
}

impl<Lang> SubstType for VariantCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
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

impl<Lang> SubstTerm for VariantPattern<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
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

impl<Lang> SubstType for VariantPattern<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        VariantPattern {
            label: self.label,
            bound_var: self.bound_var,
            rhs: Box::new(self.rhs.subst_type(v, ty)),
        }
    }
}

impl<Lang> fmt::Display for VariantCase<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pts: Vec<&VariantPattern<Lang>> = self.patterns.iter().collect();
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

impl<Lang> fmt::Display for VariantPattern<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> => {}", self.label, self.bound_var, self.rhs)
    }
}
