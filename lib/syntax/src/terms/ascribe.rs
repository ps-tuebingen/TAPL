use super::Term;
use crate::{
    TypeVar, Var,
    free_vars::{FreeTypeVars, FreeVars},
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, rc::Rc};

/// Term representing a type ascription `t:ty`
#[derive(Clone, Debug, EqNoSpan)]
pub struct Ascribe<Lang>
where
    Lang: Language,
{
    /// The ascribed term
    pub term: Rc<Lang::Term>,
    /// The ascribed type
    pub ty: Lang::Type,
    /// The source Span
    pub span: Span,
}

impl<Lang> Ascribe<Lang>
where
    Lang: Language,
{
    /// Create a new Ascription from given term, type and span
    pub fn new<T1, Ty1>(t: T1, ty: Ty1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            term: Rc::new(t.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Ascribe<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeTypeVars for Ascribe<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        self.ty.free_type_vars(vars);
        self.term.free_type_vars(vars);
    }
}

impl<Lang> FreeVars for Ascribe<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.term.free_vars(vars);
    }
}

impl<Lang> Term for Ascribe<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Ascribe<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            term: self.term.subst(v, t),
            ty: self.ty,
            span: self.span,
        }
    }
}

impl<Lang> SubstType for Ascribe<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            term: self.term.subst_type(v, ty),
            ty: self.ty.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Ascribe<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} : {})", self.term, self.ty)
    }
}
