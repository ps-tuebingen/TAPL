use super::Term;
use crate::{
    Label, TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing a case on variant types
#[derive(Clone, Debug, EqNoSpan)]
pub struct VariantCase<Lang>
where
    Lang: Language,
{
    /// The bound term
    pub bound_term: Rc<Lang::Term>,
    /// match patterns
    pub patterns: Vec<VariantPattern<Lang>>,
    /// Source location
    pub span: Span,
}

impl<Lang> VariantCase<Lang>
where
    Lang: Language,
{
    /// Create a new variant case with given bound term, patterns and source location
    pub fn new<T1>(bound: T1, pts: Vec<VariantPattern<Lang>>, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            bound_term: Rc::new(bound.into()),
            patterns: pts,
            span,
        }
    }
}

#[derive(Clone, Debug, EqNoSpan)]
pub struct VariantPattern<Lang>
where
    Lang: Language,
{
    pub label: Label,
    pub bound_var: Var,
    pub rhs: Rc<Lang::Term>,
    pub span: Span,
}

impl<Lang> VariantPattern<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(lb: &str, bound: &str, rhs: T1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            label: lb.to_owned(),
            bound_var: bound.to_owned(),
            rhs: Rc::new(rhs.into()),
            span,
        }
    }
}

impl<Lang> Spanned for VariantCase<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for VariantCase<Lang> where Lang: Language {}

impl<Lang> SubstTerm for VariantCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.bound_term = self.bound_term.subst(v, t);
        self.patterns = self.patterns.into_iter().map(|pt| pt.subst(v, t)).collect();
        self
    }
}

impl<Lang> SubstType for VariantCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.bound_term = self.bound_term.subst_type(v, ty);
        self.patterns = self
            .patterns
            .into_iter()
            .map(|pt| pt.subst_type(v, ty))
            .collect();
        self
    }
}

impl<Lang> SubstTerm for VariantPattern<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v != self.bound_var {
            self.rhs = self.rhs.subst(v, t);
        }
        self
    }
}

impl<Lang> SubstType for VariantPattern<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.rhs = self.rhs.subst_type(v, ty);
        self
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
                .map(std::string::ToString::to_string)
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
