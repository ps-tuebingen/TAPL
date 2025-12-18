use super::Term;
use crate::{
    TypeVar, Var,
    free_vars::{FreeTypeVars, FreeVars},
    kinds::Kind,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, rc::Rc};

/// Term representing a type abstraction
/// `\X::K.t`
#[derive(Clone, Debug, EqNoSpan)]
pub struct TyLambda<Lang>
where
    Lang: Language,
{
    /// Bound type variable
    pub var: TypeVar,
    /// Annotated kind
    pub annot: Kind,
    /// Body term
    pub term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> TyLambda<Lang>
where
    Lang: Language,
{
    /// Create a new tylambda with given type variable, kind annotation, body and span
    pub fn new<T1>(v: &str, knd: Kind, t: T1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            var: v.into(),
            annot: knd,
            term: Rc::new(t.into()),
            span,
        }
    }
}

impl<Lang> Spanned for TyLambda<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeVars for TyLambda<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.term.free_vars(vars);
    }
}

impl<Lang> FreeTypeVars for TyLambda<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        let contained = vars.contains(&self.var);
        self.term.free_type_vars(vars);
        if !contained {
            vars.remove(&self.var);
        }
    }
}

impl<Lang> Term for TyLambda<Lang> where Lang: Language {}

impl<Lang> SubstTerm for TyLambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.term = self.term.subst(v, t);
        self
    }
}

impl<Lang> SubstType for TyLambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        if *v != self.var {
            self.term = self.term.subst_type(v, ty);
        }
        self
    }
}

impl<Lang> fmt::Display for TyLambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.({})", self.var, self.annot, self.term)
    }
}
