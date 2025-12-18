use super::Type;
use crate::{
    TypeVar,
    kinds::Kind,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Universal type (unbounded)
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Forall<Lang>
where
    Lang: Language,
{
    /// Bound variable
    pub var: TypeVar,
    /// Kind of the variable
    pub kind: Kind,
    /// Inner type
    pub ty: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> Forall<Lang>
where
    Lang: Language,
{
    /// Create a new universal type from variable, kind, inner type and span
    pub fn new<Ty1>(v: &str, knd: Kind, ty: Ty1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            kind: knd,
            ty: Rc::new(ty.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Forall<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Forall<Lang> where Lang: Language {}

impl<Lang> SubstType for Forall<Lang>
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

impl<Lang> fmt::Display for Forall<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}::{}.{}", self.var, self.kind, self.ty)
    }
}
