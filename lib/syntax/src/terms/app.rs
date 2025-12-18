use super::{Lambda, Term};
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
    types::Unit as UnitTy,
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing an application `t1 t2`
#[derive(Clone, Debug, EqNoSpan)]
pub struct App<Lang>
where
    Lang: Language,
{
    /// The function term
    pub fun: Rc<Lang::Term>,
    /// The Argument term
    pub arg: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> App<Lang>
where
    Lang: Language,
{
    /// Construct a new application from given function and argument
    pub fn new<F, A>(f: F, a: A) -> Self
    where
        F: Into<Lang::Term> + Spanned,
        A: Into<Lang::Term> + Spanned,
    {
        let span = f.span().extend(&a.span());
        Self {
            fun: Rc::new(f.into()),
            arg: Rc::new(a.into()),
            span,
        }
    }

    /// Construct a sequence `t1;t2` as `(\_:Unit.t2) t1)`
    pub fn seq<T1, T2>(t1: T1, t2: T2) -> Self
    where
        T1: Spanned + Into<Lang::Term>,
        T2: Spanned + Into<Lang::Term>,
        Lambda<Lang>: Into<Lang::Term>,
        UnitTy<Lang>: Into<Lang::Type>,
    {
        let t1_span = t1.span();
        let t2_span = t2.span();
        let span = t1_span.extend(&t2_span);
        Self {
            fun: Rc::new(Lambda::new("_", UnitTy::new(t1_span), t2, t2_span).into()),
            arg: Rc::new(t1.into()),
            span,
        }
    }
}

impl<Lang> Spanned for App<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for App<Lang> where Lang: Language {}

impl<Lang> SubstTerm for App<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &Lang::Term) -> Self::Target {
        Self {
            fun: self.fun.subst(v, t),
            arg: self.arg.subst(v, t),
            span: self.span,
        }
    }
}
impl<Lang> SubstType for App<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            fun: self.fun.subst_type(v, ty),
            arg: self.arg.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for App<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
