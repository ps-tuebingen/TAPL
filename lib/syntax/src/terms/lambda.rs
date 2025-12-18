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

/// Term representing a lambda abstraction
/// `\x:ty.t`
#[derive(Clone, Debug, EqNoSpan)]
pub struct Lambda<Lang>
where
    Lang: Language,
{
    /// Bound variable
    pub var: Var,
    /// Type annotation for the variable
    pub annot: Lang::Type,
    /// Body term
    pub body: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Lambda<Lang>
where
    Lang: Language,
{
    /// Create a new lambda term with bound variable, type, body and span
    pub fn new<T, Ty>(v: &str, ty: Ty, t: T, span: Span) -> Self
    where
        T: Into<Lang::Term>,
        Ty: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            annot: ty.into(),
            body: Rc::new(t.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Lambda<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeVars for Lambda<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        let contained = vars.contains(&self.var);
        self.body.free_vars(vars);
        if !contained {
            vars.remove(&self.var);
        }
    }
}

impl<Lang> FreeTypeVars for Lambda<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        self.body.free_type_vars(vars);
        self.annot.free_type_vars(vars);
    }
}

impl<Lang> Term for Lambda<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Lambda<Lang>
where
    Lang: Language,
    Self: Into<Lang::Term>,
{
    type Target = Self;
    type Lang = Lang;

    fn subst(self, v: &Var, t: &<Self::Lang as Language>::Term) -> Self::Target {
        if *v == self.var {
            self
        } else {
            Self {
                var: self.var,
                annot: self.annot,
                body: self.body.subst(v, t),
                span: self.span,
            }
        }
    }
}

impl<Lang> SubstType for Lambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            var: self.var,
            annot: self.annot.subst_type(v, ty),
            body: self.body.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Lambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty_str = self.annot.to_string();
        if ty_str.is_empty() {
            write!(f, "\\{}.{}", self.var, self.body)
        } else {
            write!(f, "\\{}:{}.({})", self.var, ty_str, self.body)
        }
    }
}
