use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

/// Term representing a `Cons` list
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cons<Lang>
where
    Lang: Language,
{
    /// Head of the list
    pub head: Rc<Lang::Term>,
    /// Tail of the list
    pub tail: Rc<Lang::Term>,
    /// Annotated type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Cons<Lang>
where
    Lang: Language,
{
    /// Create a new cons term with given head, tail, type and span
    pub fn new<H, Tl, Typ>(h: H, tl: Tl, ty: Typ, span: Span) -> Self
    where
        H: Into<Lang::Term>,
        Tl: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Self {
            head: Rc::new(h.into()),
            tail: Rc::new(tl.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Cons<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Cons<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Cons<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            head: self.head.subst(v, t),
            tail: self.tail.subst(v, t),
            ty: self.ty,
            span: self.span,
        }
    }
}

impl<Lang> SubstType for Cons<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            head: self.head.subst_type(v, ty),
            tail: self.tail.subst_type(v, ty),
            ty: self.ty.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Cons<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
