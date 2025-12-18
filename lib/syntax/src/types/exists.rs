use super::{ExistsBounded, Top, Type};
use crate::{
    TypeVar,
    kinds::Kind,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Existential Type (unbounded)
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Exists<Lang>
where
    Lang: Language,
{
    /// Bound variable
    pub var: TypeVar,
    /// Kind of the type
    pub kind: Kind,
    /// Inner type
    pub ty: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> Exists<Lang>
where
    Lang: Language,
{
    /// Create a new existential type from bound variable, kind, inner type and span
    pub fn new<Ty1>(v: &str, knd: Kind, ty: Ty1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            kind: knd,
            span,
            ty: Rc::new(ty.into()),
        }
    }

    /// Convert `Self` to [`ExistsBounded`] with bound [`Top`]
    #[must_use]
    pub fn to_exists_bounded(self) -> ExistsBounded<Lang>
    where
        Top<Lang>: Into<Lang::Type>,
    {
        ExistsBounded::new(
            &self.var,
            Top::new_star(self.span),
            Rc::unwrap_or_clone(self.ty),
            self.span,
        )
    }
}

impl<Lang> Spanned for Exists<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Exists<Lang> where Lang: Language {}

impl<Lang> SubstType for Exists<Lang>
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

impl<Lang> fmt::Display for Exists<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{exists {}::{},{}}}", self.var, self.kind, self.ty)
    }
}
