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

/// Term representing an if expression
#[derive(Clone, Debug, EqNoSpan)]
pub struct If<Lang>
where
    Lang: Language,
{
    /// Condition
    pub if_cond: Rc<Lang::Term>,
    /// Then term
    pub then_term: Rc<Lang::Term>,
    /// Else term
    pub else_term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> If<Lang>
where
    Lang: Language,
{
    /// Create a new if expression with given condition, then term, else term and span
    pub fn new<T1, T2, T3>(cond: T1, th: T2, els: T3, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        T3: Into<Lang::Term>,
    {
        Self {
            if_cond: Rc::new(cond.into()),
            then_term: Rc::new(th.into()),
            else_term: Rc::new(els.into()),
            span,
        }
    }
}

impl<Lang> Spanned for If<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeVars for If<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.if_cond.free_vars(vars);
        self.then_term.free_vars(vars);
        self.else_term.free_vars(vars);
    }
}

impl<Lang> FreeTypeVars for If<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        self.if_cond.free_type_vars(vars);
        self.then_term.free_type_vars(vars);
        self.else_term.free_type_vars(vars);
    }
}

impl<Lang> Term for If<Lang> where Lang: Language {}

impl<Lang> SubstTerm for If<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            if_cond: self.if_cond.subst(v, t),
            then_term: self.then_term.subst(v, t),
            else_term: self.else_term.subst(v, t),
            span: self.span,
        }
    }
}

impl<Lang> SubstType for If<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            if_cond: self.if_cond.subst_type(v, ty),
            then_term: self.then_term.subst_type(v, ty),
            else_term: self.else_term.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for If<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.if_cond, self.then_term, self.else_term
        )
    }
}
