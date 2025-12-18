use super::Type;
use crate::{
    TypeVar,
    free_vars::FreeTypeVars,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, rc::Rc};

/// Source Type
#[derive(Clone, Debug, EqNoSpan)]
pub struct Source<Lang>
where
    Lang: Language,
{
    /// Inner type
    pub ty: Rc<Lang::Type>,
    /// Source Location
    pub span: Span,
}

impl<Lang> Source<Lang>
where
    Lang: Language,
{
    /// Create a new Source with inner type and span
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

impl<Lang> Spanned for Source<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeTypeVars for Source<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        self.ty.free_type_vars(vars);
    }
}

impl<Lang> Type for Source<Lang> where Lang: Language {}

impl<Lang> SubstType for Source<Lang>
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

impl<Lang> fmt::Display for Source<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source[{}]", self.ty)
    }
}
