use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
    types::Top,
};
use std::{fmt, rc::Rc};

/// Term representing a Type abstraction
/// `\X<:ty.t`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LambdaSub<Lang>
where
    Lang: Language,
{
    /// The bound Type variable
    pub var: TypeVar,
    /// The supertype of the variable
    pub sup_ty: Lang::Type,
    /// The body of the abstraction
    pub body: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> LambdaSub<Lang>
where
    Lang: Language,
{
    /// Create a new lambda sub term with given type variable, super type, body and span
    pub fn new<Ty, T>(v: &str, sup: Ty, bod: T, span: Span) -> Self
    where
        Ty: Into<Lang::Type>,
        T: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: sup.into(),
            body: Rc::new(bod.into()),
            span,
        }
    }

    /// Create a new lambda sub term with given type variable, body and span
    /// in this case the supertype will be [`Top`] with [`crate::kinds::Kind`] star
    pub fn new_unbounded<T>(v: &str, bod: T, span: Span) -> Self
    where
        T: Into<Lang::Term>,
        Top<Lang>: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: Top::new_star(span).into(),
            body: Rc::new(bod.into()),
            span,
        }
    }
}

impl<Lang> Spanned for LambdaSub<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for LambdaSub<Lang> where Lang: Language {}

impl<Lang> SubstTerm for LambdaSub<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v == self.var {
            self
        } else {
            Self {
                var: self.var,
                sup_ty: self.sup_ty,
                body: self.body.subst(v, t),
                span: self.span,
            }
        }
    }
}

impl<Lang> SubstType for LambdaSub<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        let sup_subst = self.sup_ty.subst_type(v, ty);
        if *v == self.var {
            Self {
                var: self.var,
                sup_ty: sup_subst,
                body: self.body,
                span: self.span,
            }
        } else {
            Self {
                var: self.var,
                sup_ty: sup_subst,
                body: self.body.subst_type(v, ty),
                span: self.span,
            }
        }
    }
}

impl<Lang> fmt::Display for LambdaSub<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:({}).{}", self.var, self.sup_ty, self.body)
    }
}
