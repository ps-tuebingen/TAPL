use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use std::{fmt, rc::Rc};

/// Reference Type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference<Lang>
where
    Lang: Language,
{
    /// Inner type
    pub ty: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> Reference<Lang>
where
    Lang: Language,
{
    /// Create a new reference type from inner type and span
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

impl<Lang> Spanned for Reference<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Reference<Lang> where Lang: Language {}

impl<Lang> SubstType for Reference<Lang>
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

impl<Lang> fmt::Display for Reference<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ref[{}]", self.ty)
    }
}
