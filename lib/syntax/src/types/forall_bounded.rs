use super::{Top, Type};
use crate::{
    TypeVar,
    free_vars::FreeTypeVars,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, rc::Rc};

/// Bounded universal type
#[derive(Clone, Debug, EqNoSpan)]
pub struct ForallBounded<Lang>
where
    Lang: Language,
{
    /// Bound variable
    pub var: TypeVar,
    /// Super type
    pub sup_ty: Rc<Lang::Type>,
    /// Inner type
    pub ty: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> ForallBounded<Lang>
where
    Lang: Language,
{
    /// Create a new bounded universal type from variable, super type, inner type and span
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

    /// Create a new bounded universal type with supertype [`crate::types::top::Top`]
    /// from varible, inner type and span
    /// kind of Top will be Star
    pub fn new_unbounded<Typ>(v: &str, ty: Typ, span: Span) -> Self
    where
        Typ: Into<Lang::Type>,
        Top<Lang>: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: Rc::new(Top::new_star(span).into()),
            ty: Rc::new(ty.into()),
            span,
        }
    }
}

impl<Lang> Spanned for ForallBounded<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeTypeVars for ForallBounded<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        let contained = vars.contains(&self.var);
        self.ty.free_type_vars(vars);
        if !contained {
            vars.remove(&self.var);
        }
        self.sup_ty.free_type_vars(vars);
    }
}

impl<Lang> Type for ForallBounded<Lang> where Lang: Language {}

impl<Lang> SubstType for ForallBounded<Lang>
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

impl<Lang> fmt::Display for ForallBounded<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}<:{}.{}", self.var, self.sup_ty, self.ty)
    }
}
