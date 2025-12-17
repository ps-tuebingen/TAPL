use super::{OpLambdaSub, Top, Type};
use crate::{
    TypeVar,
    kinds::Kind,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Operator Abstraction (unbounded)
#[derive(Clone, Debug, EqNoSpan)]
pub struct OpLambda<Lang>
where
    Lang: Language,
{
    /// bound variable
    pub var: TypeVar,
    /// Kind annotation
    pub annot: Kind,
    /// inner type
    pub body: Rc<Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> OpLambda<Lang>
where
    Lang: Language,
{
    /// Create a new operator abstraction from variable, kind, inner type and span
    pub fn new<Ty1>(var: &str, knd: Kind, ty: Ty1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            var: var.to_owned(),
            annot: knd,
            body: Rc::new(ty.into()),
            span,
        }
    }

    /// Convert `Self` to [`OpLambdaSub`] with supertype [`Top`]
    #[must_use]
    pub fn to_oplambda_unbounded(self) -> OpLambdaSub<Lang>
    where
        Top<Lang>: Into<Lang::Type>,
    {
        OpLambdaSub::new_unbounded(
            &self.var,
            self.annot,
            Rc::unwrap_or_clone(self.body),
            self.span,
        )
    }
}

impl<Lang> Spanned for OpLambda<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for OpLambda<Lang> where Lang: Language {}

impl<Lang> SubstType for OpLambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        if *v != self.var {
            self.body = self.body.subst_type(v, ty);
        }
        self
    }
}

impl<Lang> fmt::Display for OpLambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.{}", self.var, self.annot, self.body)
    }
}
