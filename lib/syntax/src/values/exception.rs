use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Exception as ExceptionT,
};
use macros::EqNoSpan;
use std::{fmt, marker::PhantomData};

/// Exception value
#[derive(Debug, EqNoSpan, Clone)]
pub struct Exception<Lang>
where
    Lang: Language,
{
    /// Annotated type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang::Term>,
}

impl<Lang> Exception<Lang>
where
    Lang: Language,
{
    /// Create a new Exception value with given type and span
    pub fn new<Ty1>(ty: Ty1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            ty: ty.into(),
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Exception<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Exception<Lang>
where
    Lang: Language,
    ExceptionT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = ExceptionT<Lang>;
}

impl<Lang> From<Exception<Lang>> for ExceptionT<Lang>
where
    Lang: Language,
{
    fn from(ex: Exception<Lang>) -> Self {
        Self::new(ex.ty, ex.span)
    }
}

impl<Lang> fmt::Display for Exception<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
