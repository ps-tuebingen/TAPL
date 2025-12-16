use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Sink Type
#[derive(Clone, Debug, EqNoSpan)]
pub struct Sink<Lang>
where
    Lang: Language,
{
    /// Inner type
    pub ty: Rc<Lang::Type>,
    /// Source Location
    pub span: Span,
}

impl<Lang> Sink<Lang>
where
    Lang: Language,
{
    /// Create a new Sink with inner type and span
    pub fn new<Ty1>(ty: Ty1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            ty: Rc::new(ty.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Sink<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Sink<Lang> where Lang: Language {}

impl<Lang> SubstType for Sink<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.ty = self.ty.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Sink<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sink[{}]", self.ty)
    }
}
