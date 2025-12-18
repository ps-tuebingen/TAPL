use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Raise as RaiseT,
};
use macros::{EqNoSpan, HashNoSpan};
use std::fmt;

/// Raise value
#[derive(HashNoSpan, Debug, EqNoSpan, Clone)]
pub struct Raise<Lang>
where
    Lang: Language,
{
    /// inner value
    pub val: Box<Lang::Value>,
    /// continuation type
    pub cont_ty: Lang::Type,
    /// Exception type
    pub exception_ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Raise<Lang>
where
    Lang: Language,
{
    /// Create a new raise value with given inner value, continuation and exception types and span
    pub fn new<V1, Ty1, Ty2>(v: V1, cont_ty: Ty1, ex_ty: Ty2, span: Span) -> Self
    where
        V1: Into<Lang::Value>,
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            val: Box::new(v.into()),
            cont_ty: cont_ty.into(),
            exception_ty: ex_ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Raise<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Raise<Lang>
where
    Lang: Language,
    RaiseT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = RaiseT<Lang>;
}

impl<Lang> From<Raise<Lang>> for RaiseT<Lang>
where
    Lang: Language,
{
    fn from(r: Raise<Lang>) -> Self {
        Self::new(*r.val, r.exception_ty, r.cont_ty, r.span)
    }
}

impl<Lang> fmt::Display for Raise<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{}]({} : {})",
            self.cont_ty, self.val, self.exception_ty
        )
    }
}
