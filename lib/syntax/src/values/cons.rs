use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Cons as ConsT,
    values::Value,
};
use macros::{EqNoSpan, HashNoSpan};
use std::fmt;

/// List Value
#[derive(HashNoSpan, Debug, EqNoSpan, Clone)]
pub struct Cons<Lang>
where
    Lang: Language,
{
    /// List head
    pub head: Box<Lang::Value>,
    /// List tail
    pub tail: Box<Lang::Value>,
    /// Type annotation
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Cons<Lang>
where
    Lang: Language,
{
    /// Create a new cons value with given head, tail, type and span
    pub fn new<V1, V2, Typ>(hd: V1, tl: V2, ty: Typ, span: Span) -> Self
    where
        V1: Into<Lang::Value>,
        V2: Into<Lang::Value>,
        Typ: Into<Lang::Type>,
    {
        Self {
            head: Box::new(hd.into()),
            tail: Box::new(tl.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Cons<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Cons<Lang>
where
    Lang: Language,
    ConsT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = ConsT<Lang>;
}

impl<Lang> From<Cons<Lang>> for ConsT<Lang>
where
    Lang: Language,
{
    fn from(c: Cons<Lang>) -> Self {
        Self::new(*c.head, *c.tail, c.ty, c.span)
    }
}

impl<Lang> fmt::Display for Cons<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
