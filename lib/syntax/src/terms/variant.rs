use super::Term;
use crate::{
    Label, TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing a variant
#[derive(Clone, Debug, EqNoSpan)]
pub struct Variant<Lang>
where
    Lang: Language,
{
    /// Variant label
    pub label: Label,
    /// Term corresponding to the label
    pub term: Rc<Lang::Term>,
    /// Type annotation
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Variant<Lang>
where
    Lang: Language,
{
    /// Create a new variant with given label, term, type and span
    pub fn new<T1, Ty1>(lb: &str, t: T1, ty: Ty1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            label: lb.to_owned(),
            term: Rc::new(t.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Variant<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Variant<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Variant<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            label: self.label,
            term: self.term.subst(v, t),
            ty: self.ty,
            span: self.span,
        }
    }
}

impl<Lang> SubstType for Variant<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.term = self.term.subst_type(v, ty);
        self.ty = self.ty.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Variant<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}={}> as {}", self.label, self.term, self.ty)
    }
}
