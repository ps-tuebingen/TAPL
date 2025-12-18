use super::{Top, Type};
use crate::{
    TypeVar,
    kinds::Kind,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Bounded existential type
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct ExistsBounded<Lang>
where
    Lang: Language,
{
    /// Bound variable
    pub var: TypeVar,
    /// Variable supertype
    pub sup_ty: Rc<Lang::Type>,
    /// Inner type
    pub ty: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> ExistsBounded<Lang>
where
    Lang: Language,
{
    /// Create a new bounded existential type from variable, super type, inner type and span
    pub fn new<Ty1, Ty2>(v: &str, sup: Ty1, ty: Ty2, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: Rc::new(sup.into()),
            ty: Rc::new(ty.into()),
            span,
        }
    }

    /// Create a new bounded existential type with supertype [`crate::types::top::Top`]
    /// from bound variable, kind of Top, inner type and span
    pub fn new_unbounded<Ty1>(v: &str, knd: Kind, ty: Ty1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        Top<Lang>: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: Rc::new(Top::new(knd, span).into()),
            ty: Rc::new(ty.into()),
            span,
        }
    }
}

impl<Lang> Spanned for ExistsBounded<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for ExistsBounded<Lang> where Lang: Language {}

impl<Lang> SubstType for ExistsBounded<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;

    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.sup_ty = self.sup_ty.subst_type(v, ty);
        if *v != self.var {
            self.ty = self.ty.subst_type(v, ty);
        }
        self
    }
}

impl<Lang> fmt::Display for ExistsBounded<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{exists {}<:{},{}}}", self.var, self.sup_ty, self.ty)
    }
}
