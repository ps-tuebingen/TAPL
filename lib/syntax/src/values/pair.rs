use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Pair as PairT,
};
use std::fmt;

/// Pair value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair<Lang>
where
    Lang: Language,
{
    /// first value
    pub fst: Box<Lang::Value>,
    /// second value
    pub snd: Box<Lang::Value>,
    /// Source location
    pub span: Span,
}

impl<Lang> Pair<Lang>
where
    Lang: Language,
{
    /// Create a new pair value from first and second value and span
    pub fn new<V1, V2>(fst: V1, snd: V2, span: Span) -> Self
    where
        V1: Into<Lang::Value>,
        V2: Into<Lang::Value>,
    {
        Self {
            fst: Box::new(fst.into()),
            snd: Box::new(snd.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Pair<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Pair<Lang>
where
    Lang: Language,
    PairT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = PairT<Lang>;
}

impl<Lang> From<Pair<Lang>> for PairT<Lang>
where
    Lang: Language,
{
    fn from(p: Pair<Lang>) -> Self {
        Self::new(*p.fst, *p.snd, p.span)
    }
}

impl<Lang> fmt::Display for Pair<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
