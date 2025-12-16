use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Operator Application Type
#[derive(Clone, Debug, EqNoSpan)]
pub struct OpApp<Lang>
where
    Lang: Language,
{
    /// Operator function type
    pub fun: Rc<Lang::Type>,
    /// Operator argument type
    pub arg: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> OpApp<Lang>
where
    Lang: Language,
{
    /// Create a new operator abstraction from function, argument and span
    pub fn new<Ty1, Ty2>(fun: Ty1, arg: Ty2, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            fun: Rc::new(fun.into()),
            arg: Rc::new(arg.into()),
            span,
        }
    }
}

impl<Lang> Spanned for OpApp<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for OpApp<Lang> where Lang: Language {}

impl<Lang> SubstType for OpApp<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.fun = self.fun.subst_type(v, ty);
        self.arg = self.arg.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for OpApp<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}[{}])", self.fun, self.arg)
    }
}
