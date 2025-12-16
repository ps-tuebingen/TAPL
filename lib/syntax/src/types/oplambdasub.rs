use super::{Top, Type};
use crate::{
    TypeVar,
    kinds::Kind,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use std::{fmt, rc::Rc};

/// Bounded Operator Abstraction
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpLambdaSub<Lang>
where
    Lang: Language,
{
    /// bound variable
    pub var: TypeVar,
    /// Super type
    pub sup: Rc<Lang::Type>,
    /// inner Type
    pub body: Rc<Lang::Type>,
    /// Source Location
    pub span: Span,
}

impl<Lang> OpLambdaSub<Lang>
where
    Lang: Language,
{
    /// Create a new bounded operator abstraction from variable, supertype, inner type and span
    pub fn new<Ty1, Ty2>(var: &str, sup: Ty1, ty: Ty2, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            var: var.to_owned(),
            sup: Rc::new(sup.into()),
            body: Rc::new(ty.into()),
            span,
        }
    }

    /// Create a new bounded operator abstraction with supertype [`crate::types::top::Top`]
    /// from bound variable kind of Top, inner type and span
    pub fn new_unbounded<Ty1>(var: &str, knd: Kind, ty: Ty1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        Top<Lang>: Into<Lang::Type>,
    {
        Self {
            var: var.to_owned(),
            sup: Rc::new(Top::new(knd, span).into()),
            body: Rc::new(ty.into()),
            span,
        }
    }
}

impl<Lang> Spanned for OpLambdaSub<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for OpLambdaSub<Lang> where Lang: Language {}

impl<Lang> SubstType for OpLambdaSub<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.sup = self.sup.subst_type(v, ty);
        if *v != self.var {
            self.body = self.body.subst_type(v, ty);
        }
        self
    }
}

impl<Lang> fmt::Display for OpLambdaSub<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup, self.body)
    }
}
