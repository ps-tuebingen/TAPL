use super::Term;
use crate::{
    TypeVar, Var,
    free_vars::{FreeTypeVars, FreeVars},
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, marker::PhantomData};

/// Term representing a variable
#[derive(Clone, Debug, EqNoSpan)]
pub struct Variable<Lang>
where
    Lang: Language,
{
    /// The variable
    pub var: Var,
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Variable<Lang>
where
    Lang: Language,
{
    /// Create a new Variable with given variable and span
    #[must_use]
    pub fn new(v: &str, span: Span) -> Self {
        Self {
            var: v.to_owned(),
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Variable<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeVars for Variable<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        vars.insert(self.var.clone());
    }
}

impl<Lang> FreeTypeVars for Variable<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, _: &mut HashSet<TypeVar>) {
        ()
    }
}

impl<Lang> Term for Variable<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Variable<Lang>
where
    Lang: Language,
    Self: Into<Lang::Term>,
{
    type Target = Lang::Term;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v == self.var {
            t.clone()
        } else {
            self.into()
        }
    }
}

impl<Lang> SubstType for Variable<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Variable<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}
