use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Sum type
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Sum<Lang>
where
    Lang: Language,
{
    /// Left Type
    pub left: Rc<Lang::Type>,
    /// Right Type
    pub right: Rc<Lang::Type>,
    /// Source Location
    pub span: Span,
}

impl<Lang> Sum<Lang>
where
    Lang: Language,
{
    /// Create a new Sum with left and right types and span
    pub fn new<Ty1, Ty2>(l: Ty1, r: Ty2, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            left: Rc::new(l.into()),
            right: Rc::new(r.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Sum<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Sum<Lang> where Lang: Language {}

impl<Lang> SubstType for Sum<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Self;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.left = self.left.subst_type(v, ty);
        self.right = self.right.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Sum<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}+{})", self.left, self.right)
    }
}
