use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Product type
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Product<Lang>
where
    Lang: Language,
{
    /// First type
    pub fst: Rc<Lang::Type>,
    /// Second type
    pub snd: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> Product<Lang>
where
    Lang: Language,
{
    /// Create a new product type from first and second types and span
    pub fn new<Ty1, Ty2>(fst: Ty1, snd: Ty2, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            fst: Rc::new(fst.into()),
            snd: Rc::new(snd.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Product<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Product<Lang> where Lang: Language {}

impl<Lang> SubstType for Product<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Target = Self;
    type Lang = Lang;

    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.fst = self.fst.subst_type(v, ty);
        self.snd = self.snd.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Product<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} x {})", self.fst, self.snd)
    }
}
