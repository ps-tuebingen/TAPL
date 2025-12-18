use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Function Type
#[derive(Clone, Debug, EqNoSpan)]
pub struct Fun<Lang>
where
    Lang: Language,
{
    /// From type
    pub from: Rc<Lang::Type>,
    /// To type
    pub to: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> Fun<Lang>
where
    Lang: Language,
{
    /// Create a new function type from from, to and span
    pub fn new<Ty1, Ty2>(from: Ty1, to: Ty2, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            from: Rc::new(from.into()),
            to: Rc::new(to.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Fun<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Fun<Lang> where Lang: Language {}

impl<Lang> SubstType for Fun<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.from = self.from.subst_type(v, ty);
        self.to = self.to.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Fun<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} -> {})", self.from, self.to)
    }
}
