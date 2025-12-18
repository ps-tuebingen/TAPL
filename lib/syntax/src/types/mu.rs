use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Recursive type
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Mu<Lang>
where
    Lang: Language,
{
    /// Bound variable
    pub var: TypeVar,
    /// Inner type
    pub ty: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> Mu<Lang>
where
    Lang: Language,
{
    /// Create a new recursive type from bound variable, inner type and span
    pub fn new<Ty1>(v: &str, ty: Ty1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            ty: Rc::new(ty.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Mu<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Mu<Lang> where Lang: Language {}

impl<Lang> SubstType for Mu<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        if *v != self.var {
            self.ty = self.ty.subst_type(v, ty);
        }
        self
    }
}

impl<Lang> fmt::Display for Mu<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mu {}.{}", self.var, self.ty)
    }
}
