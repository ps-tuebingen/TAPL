use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// term representing raising an exception
/// used with [`crate::terms::tryval::TryWithVal`]
#[derive(Clone, Debug, EqNoSpan)]
pub struct Raise<Lang>
where
    Lang: Language,
{
    /// The exception term
    pub exception: Rc<Lang::Term>,
    /// Type of the execption
    pub exception_ty: Lang::Type,
    /// Type of the continuation
    pub cont_ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Raise<Lang>
where
    Lang: Language,
{
    /// Create a new raise term with given exception, exception and continuation types and span
    pub fn new<E, Ty1, Ty2>(ex: E, ex_ty: Ty1, cont_ty: Ty2, span: Span) -> Self
    where
        E: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            exception: Rc::new(ex.into()),
            exception_ty: ex_ty.into(),
            cont_ty: cont_ty.into(),
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

impl<Lang> Term for Raise<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Raise<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.exception = self.exception.subst(v, t);
        self
    }
}

impl<Lang> SubstType for Raise<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.exception = self.exception.subst_type(v, ty);
        self.exception_ty = self.exception_ty.subst_type(v, ty);
        self.cont_ty = self.cont_ty.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Raise<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{},{}]({})",
            self.cont_ty, self.exception_ty, self.exception,
        )
    }
}
