use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// List type
#[derive(Clone, Debug, EqNoSpan)]
pub struct List<Lang>
where
    Lang: Language,
{
    /// Type argument
    pub ty: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> List<Lang>
where
    Lang: Language,
{
    /// Create a new list type from argument and span
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

impl<Lang> Spanned for List<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for List<Lang> where Lang: Language {}

impl<Lang> SubstType for List<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.ty = self.ty.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for List<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List[{}]", self.ty)
    }
}
