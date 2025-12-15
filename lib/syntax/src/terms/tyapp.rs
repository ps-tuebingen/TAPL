use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

/// Term representing a type application
/// `t [ty]`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyApp<Lang>
where
    Lang: Language,
{
    ///Term to apply to
    pub fun: Rc<Lang::Term>,
    /// Applied type
    pub arg: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> TyApp<Lang>
where
    Lang: Language,
{
    /// Create a new type application with given term, type and span
    pub fn new<T1, Typ>(t: T1, ty: Typ, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Self {
            fun: Rc::new(t.into()),
            arg: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for TyApp<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for TyApp<Lang> where Lang: Language {}

impl<Lang> SubstTerm for TyApp<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.fun = self.fun.subst(v, t);
        self
    }
}

impl<Lang> SubstType for TyApp<Lang>
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

impl<Lang> fmt::Display for TyApp<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(({})[{}])", self.fun, self.arg)
    }
}
